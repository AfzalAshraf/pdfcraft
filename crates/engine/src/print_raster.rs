//! Print as image (Acrobat's Advanced Print Setup ▸ Print As Image): every page
//! sent to the printer as a picture at a chosen resolution, for printers that skip text,
//! shading or transparency. The pages are rendered the way the viewer shows them (reading
//! colours off, the print's own colours), made into an image-only document, and that document
//! is imposed on the paper like any other. The print is no longer searchable text.

use pdfcraft_cos::{Dict, Object, Stream};
use pdfcraft_print::Raster;
use pdfcraft_render::{PRINT_TAG, PageRenderer, RenderRequest};

use crate::Document;
use crate::export::encode_png;

// ISO 32000-2 Annex C: these image pages use default user units (one point).
const MAX_PAGE_SIDE: f64 = 14_400.0;

fn valid_page_size(w: f64, h: f64) -> bool {
    w.is_finite() && h.is_finite() && w > 0.0 && h > 0.0 && w <= MAX_PAGE_SIDE && h <= MAX_PAGE_SIDE
}

/// `doc`'s pages `pages` (0-based, each once, in the order given) as an image-only PDF, one
/// page per entry, the size of the source page as displayed. Pages too large for the renderer
/// at `raster.dpi` render at the largest resolution it allows. Displayed sides over 200 inches
/// are refused rather than silently resized or written outside default-unit PDF page limits.
pub fn rasterize(doc: &Document, pages: &[usize], raster: &Raster, hide_comments: bool) -> Result<Vec<u8>, String> {
    if !raster.dpi.is_finite() || !(72.0..=600.0).contains(&raster.dpi) {
        return Err("print DPI must be finite and between 72 and 600".into());
    }
    if pages.is_empty() || pages.len() > 10_000 {
        return Err("print as image requires between 1 and 10000 pages".into());
    }
    let dpi = raster.dpi;
    let src = doc.export_source();
    let mut config = src.config.clone();
    config.hide_comments = hide_comments;
    let mut renderer = PageRenderer::new(src.bytes, config);
    let mut images: Vec<(String, Vec<u8>)> = Vec::with_capacity(pages.len());
    let mut sizes = Vec::with_capacity(pages.len());
    for &page in pages {
        let (w, h) = src.sizes.get(page).copied().ok_or_else(|| format!("page {} does not exist", page.saturating_add(1)))?;
        if !valid_page_size(f64::from(w), f64::from(h)) {
            return Err(format!("page {} exceeds image-print page limits; use vector printing instead", page.saturating_add(1)));
        }
        let scale = pdfcraft_render::effective_scale(w, h, (dpi / 72.0) as f32);
        sizes.push((f64::from(w), f64::from(h)));
        let r = renderer.render(RenderRequest { page, scale, tag: PRINT_TAG, ..Default::default() });
        if let Some(e) = r.error {
            return Err(format!("page {}: {e}", page.saturating_add(1)));
        }
        let mut rgba = r.rgba.to_vec();
        over_white(&mut rgba, raster.grayscale);
        images.push((format!("page-{}.png", page.saturating_add(1)), encode_png(r.width, r.height, &rgba)?));
    }
    let doc = image_pages(&images, &sizes)?;
    pdfcraft_cos::write_full(&doc, &pdfcraft_cos::SaveOptions::default()).map_err(|e| e.to_string())
}

/// Place each raster on the exact displayed source rectangle, independently of pixel count.
/// A capped large page must not enlarge other pages or trigger another fit-to-paper reduction.
fn image_pages(images: &[(String, Vec<u8>)], sizes: &[(f64, f64)]) -> Result<pdfcraft_cos::Document, String> {
    if images.is_empty() || images.len() != sizes.len() {
        return Err("print images and page sizes must be nonempty and match".into());
    }
    let mut doc = pdfcraft_cos::Document::new_empty();
    let root = doc.root().ok_or("the print document has no catalog")?;
    let parent = doc.get(root).as_dict().and_then(|d| d.reference(b"Pages")).ok_or("the print document has no page tree")?;
    let mut kids = Vec::with_capacity(images.len());
    for ((name, bytes), &(w, h)) in images.iter().zip(sizes) {
        if !valid_page_size(w, h) {
            return Err("print page dimensions must be finite, positive and at most 14400 points".into());
        }
        let (image, _) = pdfcraft_create::image_xobject(&mut doc, name, bytes).map_err(|e| e.to_string())?;
        let mut xobjects = Dict::new();
        xobjects.set(b"Im0".to_vec(), Object::Ref(image));
        let mut resources = Dict::new();
        resources.set(b"XObject".to_vec(), Object::Dict(xobjects));
        let content = format!("q {w:.6} 0 0 {h:.6} 0 0 cm /Im0 Do Q\n");
        let content = doc.add(Object::Stream(Stream::flate(Dict::new(), content.as_bytes())));
        let mut page = Dict::new();
        page.set(b"Type".to_vec(), Object::name("Page"));
        page.set(b"Parent".to_vec(), Object::Ref(parent));
        page.set(b"MediaBox".to_vec(), Object::Array(vec![0.into(), 0.into(), Object::Real(w), Object::Real(h)]));
        page.set(b"Resources".to_vec(), Object::Dict(resources));
        page.set(b"Contents".to_vec(), Object::Ref(content));
        kids.push(Object::Ref(doc.add(page)));
    }
    let count = i64::try_from(kids.len()).map_err(|_| "too many print pages")?;
    doc.update_dict(parent, |d| {
        d.set(b"Kids".to_vec(), Object::Array(kids));
        d.set(b"Count".to_vec(), Object::Int(count));
    })
    .map_err(|e| e.to_string())?;
    Ok(doc)
}

/// Premultiplied RGBA composed over white paper (opaque), in grey when asked (Rec. 601 luma).
fn over_white(rgba: &mut [u8], grayscale: bool) {
    for px in rgba.as_chunks_mut::<4>().0 {
        let gap = 255 - u32::from(px[3]);
        let (r, g, b) = ((u32::from(px[0]) + gap).min(255), (u32::from(px[1]) + gap).min(255), (u32::from(px[2]) + gap).min(255));
        let (r, g, b) = if grayscale {
            let y = ((299 * r + 587 * g + 114 * b) / 1000).min(255);
            (y, y, y)
        } else {
            (r, g, b)
        };
        px[0] = r as u8;
        px[1] = g as u8;
        px[2] = b as u8;
        px[3] = 255;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn capped_rasters_keep_each_source_page_size_after_save_and_reopen() {
        // The narrow second page hits MAX_SIDE without a large test allocation. It must not
        // change the first page's physical size or its independently requested resolution.
        let mut source = pdfcraft_create::blank(72.25, 72.5, 1).unwrap();
        pdfcraft_organize::insert_blank_page(&mut source, 1, 10_000.0, 3.0).unwrap();
        let bytes = pdfcraft_cos::write_full(&source, &Default::default()).unwrap();
        let mut session = crate::Session::new();
        let id = session.open("mixed.pdf", None, Arc::new(bytes), None).unwrap();
        let doc = session.get(id).unwrap();
        let bytes = rasterize(doc, &[0, 1], &Raster { dpi: 150.0, grayscale: false }, true).unwrap();
        let printed = pdfcraft_cos::Document::open(Arc::new(bytes)).unwrap();
        let sizes: Vec<_> = pdfcraft_model::pages(&printed).iter().map(|p| p.display_size(&printed)).collect();
        assert_eq!(sizes, [(72.25, 72.5), (10_000.0, 3.0)]);
        let widths: Vec<_> = printed
            .scan_objects()
            .filter_map(|(_, o)| {
                let Object::Stream(stream) = &*o else { return None };
                (stream.dict.name(b"Subtype") == Some(b"Image")).then(|| stream.dict.int(b"Width")).flatten()
            })
            .collect();
        assert!(widths.contains(&151), "the small page keeps 150 dpi: {widths:?}");
        assert!(widths.contains(&(pdfcraft_render::MAX_SIDE as i64)), "the large page respects the renderer cap: {widths:?}");
        for dpi in [f64::NAN, f64::INFINITY, 0.0, 601.0] {
            assert!(rasterize(doc, &[0], &Raster { dpi, grayscale: false }, true).unwrap_err().contains("DPI"));
        }
    }

    #[test]
    fn paper_compositing_keeps_opaque_black_and_colour_unchanged() {
        let mut rgba = [0, 0, 0, 255, 20, 40, 60, 255, 0, 0, 0, 0, 64, 0, 0, 128];
        over_white(&mut rgba, false);
        assert_eq!(rgba, [0, 0, 0, 255, 20, 40, 60, 255, 255, 255, 255, 255, 191, 127, 127, 255]);
        let mut black = [0, 0, 0, 255];
        over_white(&mut black, true);
        assert_eq!(black, [0, 0, 0, 255]);
    }

    #[test]
    fn image_pages_refuses_empty_mismatched_and_invalid_sizes() {
        let png = encode_png(1, 1, &[0, 0, 0, 255]).unwrap();
        let images = vec![("pixel.png".into(), png)];
        assert!(image_pages(&[], &[]).is_err());
        assert!(image_pages(&images, &[]).is_err());
        for size in [(f64::NAN, 1.0), (1.0, f64::INFINITY), (0.0, 1.0), (1.0, -1.0), (14_401.0, 1.0)] {
            assert!(image_pages(&images, &[size]).is_err());
        }
    }
}
