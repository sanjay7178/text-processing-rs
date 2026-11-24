/// Sentence-level inverse text normalization.
#[pyo3::pymodule]
mod text_processing_rs {
    use pyo3::prelude::*;

    /// Convert a spoken-form sentence to written text.
    ///
    /// Language codes are en, de, es, fr, hi, ja, and zh. An unrecognized code
    /// falls back to English.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en"))]
    fn normalize_sentence_lang(py: Python<'_>, text: &str, lang: &str) -> String {
        py.detach(|| crate::normalize_sentence_lang(text, lang))
    }
}
