//! This repository contains Benchmarks for dwrite.dll
//! Please use `cargo bench` instead of `cargo run`

use std::ptr::{null, null_mut};

use winapi::{um::dwrite::*, Interface};

pub unsafe fn create_factory() -> *mut IDWriteFactory {
    let iddwritefactory_guid = IDWriteFactory::uuidof();
    let mut ptr: *mut IDWriteFactory = null_mut();
    let result = DWriteCreateFactory(
        DWRITE_FACTORY_TYPE_SHARED,
        &iddwritefactory_guid,
        &mut ptr as *mut _ as *mut _,
    );
    debug_assert!(result == 0);
    ptr
}

pub unsafe fn create_text_format(
    factory: *mut IDWriteFactory,
    font_family_name: *const u16,
    locale_name: *const u16,
) -> *mut IDWriteTextFormat {
    let mut ptr: *mut IDWriteTextFormat = null_mut();
    let result = factory.as_ref().unwrap().CreateTextFormat(
        font_family_name,
        null_mut(),
        DWRITE_FONT_WEIGHT_NORMAL,
        DWRITE_FONT_STYLE_NORMAL,
        DWRITE_FONT_STRETCH_NORMAL,
        72.0,
        locale_name,
        &mut ptr,
    );
    debug_assert!(result == 0);

    ptr
}

pub unsafe fn create_number_substituion(
    factory: *mut IDWriteFactory,
    locale_name: *const u16,
) -> *mut IDWriteNumberSubstitution {
    let mut ptr: *mut IDWriteNumberSubstitution = null_mut();
    let result = factory.as_ref().unwrap().CreateNumberSubstitution(
        DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
        locale_name,
        0,
        &mut ptr,
    );
    debug_assert!(result == 0);

    ptr
}

pub unsafe fn create_font_file(
    factory: *mut IDWriteFactory,
    file_path: *const u16,
) -> *mut IDWriteFontFile {
    let mut ptr: *mut IDWriteFontFile = null_mut();
    let result = factory
        .as_ref()
        .unwrap()
        .CreateFontFileReference(file_path, null(), &mut ptr);
    debug_assert!(result == 0);

    ptr
}

pub unsafe fn create_font_face(
    factory: *mut IDWriteFactory,
    font_file: *const *mut IDWriteFontFile,
) -> *mut IDWriteFontFace {
    let mut ptr: *mut IDWriteFontFace = null_mut();
    let result = factory.as_ref().unwrap().CreateFontFace(
        DWRITE_FONT_FACE_TYPE_TRUETYPE,
        1,
        font_file,
        0,
        DWRITE_FONT_SIMULATIONS_NONE,
        &mut ptr,
    );
    debug_assert!(result == 0);
    ptr
}

pub unsafe fn create_text_analyzer(factory: *mut IDWriteFactory) -> *mut IDWriteTextAnalyzer {
    let mut ptr: *mut IDWriteTextAnalyzer = null_mut();
    let result = factory.as_ref().unwrap().CreateTextAnalyzer(&mut ptr);
    debug_assert!(result == 0);

    ptr
}

pub unsafe fn analyzer_get_glyphs(
    analyzer: *mut IDWriteTextAnalyzer,
    text: *mut u16,
    text_length: u32,
    font_face: *mut IDWriteFontFace,
    locale_name: *const u16,
    number_substituion: *mut IDWriteNumberSubstitution,
) {
    let text_props: *mut DWRITE_SHAPING_TEXT_PROPERTIES =
        &mut DWRITE_SHAPING_TEXT_PROPERTIES { bit_fields: 0 };
    let mut typographic_features: *const DWRITE_TYPOGRAPHIC_FEATURES =
        &DWRITE_TYPOGRAPHIC_FEATURES {
            featureCount: 1,
            features: DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES as *mut _,
        };
    let glyph_indices = &mut 0;
    let glyph_props: *mut DWRITE_SHAPING_GLYPH_PROPERTIES =
        &mut DWRITE_SHAPING_GLYPH_PROPERTIES { bit_fields: 0 };
    let actual_glyph_count = &mut 0;
    let result = analyzer.as_ref().unwrap().GetGlyphs(
        text,
        text_length,
        font_face,
        0,
        0,
        &DWRITE_SCRIPT_ANALYSIS {
            script: 0,
            shapes: DWRITE_SCRIPT_SHAPES_DEFAULT,
        },
        locale_name,
        number_substituion,
        &mut typographic_features,
        &0,
        0,
        64,
        &mut 0,
        text_props,
        glyph_indices,
        glyph_props,
        actual_glyph_count,
    );
    debug_assert!(result == 0);
}

pub unsafe fn analyzer_get_glyph_placements(
    analyzer: *mut IDWriteTextAnalyzer,
    text: *mut u16,
    text_length: u32,
    font_face: *mut IDWriteFontFace,
    locale_name: *const u16,
) {
    let cluster_map = &mut 0;
    let text_props: *mut DWRITE_SHAPING_TEXT_PROPERTIES =
        &mut DWRITE_SHAPING_TEXT_PROPERTIES { bit_fields: 0 };
    let glyph_indices = &mut 0;
    let glyph_props: *mut DWRITE_SHAPING_GLYPH_PROPERTIES =
        &mut DWRITE_SHAPING_GLYPH_PROPERTIES { bit_fields: 0 };
    let mut typographic_features: *const DWRITE_TYPOGRAPHIC_FEATURES =
        &DWRITE_TYPOGRAPHIC_FEATURES {
            featureCount: 1,
            features: DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES as *mut _,
        };
    let result = analyzer.as_ref().unwrap().GetGlyphPlacements(
        text,
        cluster_map,
        text_props,
        text_length,
        glyph_indices,
        glyph_props,
        64,
        font_face,
        26.0,
        0,
        0,
        &DWRITE_SCRIPT_ANALYSIS {
            script: 0,
            shapes: DWRITE_SCRIPT_SHAPES_DEFAULT,
        },
        locale_name,
        &mut typographic_features,
        &0,
        0,
        &mut 0.0,
        &mut DWRITE_GLYPH_OFFSET {
            advanceOffset: 0.0,
            ascenderOffset: 0.0,
        },
    );
    debug_assert!(result == 0);
}
