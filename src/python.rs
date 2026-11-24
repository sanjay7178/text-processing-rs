/// Sentence-level inverse text normalization (ITN) and text normalization (TN).
#[pyo3::pymodule]
mod text_processing_rs {
    use pyo3::prelude::*;

    /// Convert a spoken sentence to written form using inverse text normalization (ITN).
    ///
    /// Language codes are en, de, es, fr, hi, ja, and zh. An unrecognized code
    /// falls back to English.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en"))]
    fn itn_normalize_sentence_lang(py: Python<'_>, text: &str, lang: &str) -> String {
        py.detach(|| crate::normalize_sentence_lang(text, lang))
    }

    /// Convert a written sentence to spoken form using text normalization (TN).
    ///
    /// Language support follows the active Rust text normalization engine.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en"))]
    fn tn_normalize_sentence_lang(py: Python<'_>, text: &str, lang: &str) -> String {
        py.detach(|| crate::tn_normalize_sentence_lang(text, lang))
    }

    #[pymodule_init]
    fn add_itn_alias(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add(
            "normalize_sentence_lang",
            module.getattr("itn_normalize_sentence_lang")?,
        )
    }
}
