use criterion::{criterion_group, criterion_main, Criterion};
use dwrite_benchmarks::*;
use std::{ffi::OsStr, os::windows::prelude::OsStrExt};

const FONT_FILE: &str = "OpenSans-Regular.ttf";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("DWriteCreateFactory", |b| {
        b.iter(|| unsafe { create_factory() })
    });
    c.bench_function("IDWriteFactory.CreateTextFormat", |b| {
        let factory = unsafe { create_factory() };
        let font_family_name = utf16_str("Times New Roman").as_ptr();
        let locale_name = utf16_str("en-us").as_ptr();
        b.iter(|| unsafe { create_text_format(factory, font_family_name, locale_name) });
    });
    c.bench_function("IDWriteFactory.CreateNumberSubstitution", |b| {
        let factory = unsafe { create_factory() };
        let locale_name = utf16_str("en-us").as_ptr();
        b.iter(|| unsafe { create_number_substituion(factory, locale_name) });
    });
    c.bench_function("IDWriteFactory.CreateFontFileReference", |b| {
        let factory = unsafe { create_factory() };
        let font_file_name_ptr = utf16_str(FONT_FILE).as_ptr();
        b.iter(|| unsafe { create_font_file(factory, font_file_name_ptr) });
    });
    c.bench_function("IDWriteFactory.CreateFontFace", |b| {
        let factory = unsafe { create_factory() };
        let font_file_name_ptr = utf16_str(FONT_FILE).as_ptr();
        let font_file = unsafe { create_font_file(factory, font_file_name_ptr) };

        b.iter(|| unsafe { create_font_face(factory, &font_file) });
    });
    c.bench_function("IDWriteFactory.CreateTextAnalyzer", |b| {
        let factory = unsafe { create_factory() };
        b.iter(|| unsafe { create_text_analyzer(factory) });
    });

    // TODO / FIXME: Segmentation fault
    /*c.bench_function("IDWriteTextAnalyzer.GetGlyphs", |b| {
        let factory = unsafe { create_factory() };
        let analyzer = unsafe { create_text_analyzer(factory) };
        let locale_name = utf16_str("en-us").as_ptr();
        let font_file_name_ptr = utf16_str(FONT_FILE).as_ptr();
        let font_file = unsafe { create_font_file(factory, font_file_name_ptr) };
        let font_face = unsafe { create_font_face(factory, &font_file) };

        let number_substition = unsafe { create_number_substituion(factory, locale_name) };

        let text = "Lorem ipsum dolor sit amet";
        let text_ptr = utf16_str(text).as_mut_ptr();
        let text_len = text.len() as u32;

        b.iter(|| unsafe {
            analyzer_get_glyphs(
                analyzer,
                text_ptr,
                text_len,
                font_face,
                locale_name,
                number_substition,
            )
        });
    });*/

    // TODO / FIXME: Segmentation fault
    /*c.bench_function("IDWriteTextAnalyzer.GetGlyphPlacements", |b| {
        let factory = unsafe { create_factory() };
        let analyzer = unsafe { create_text_analyzer(factory) };
        let locale_name = utf16_str("en-us").as_ptr();
        let font_file_name_ptr = utf16_str(FONT_FILE).as_ptr();
        let font_file = unsafe { create_font_file(factory, font_file_name_ptr) };
        let font_face = unsafe { create_font_face(factory, &font_file) };

        let text = "Lorem ipsum dolor sit amet";
        let text_ptr = utf16_str(text).as_mut_ptr();
        let text_len = text.len() as u32;

        b.iter(|| unsafe {
            analyzer_get_glyph_placements(analyzer, text_ptr, text_len, font_face, locale_name);
        });
    });*/
}

fn utf16_str(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
