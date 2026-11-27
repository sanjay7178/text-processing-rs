/// Sentence-level inverse text normalization (ITN) and text normalization (TN).
#[pyo3::pymodule]
mod text_processing_rs {
    use pyo3::prelude::*;

    /// Convert a spoken sentence to written form using inverse text normalization (ITN).
    ///
    /// Language codes are en, de, es, fr, hi, ja, and zh. An unrecognized code
    /// falls back to English. Set `use_fst` to `False` to use the rule-based
    /// engine for this call.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en", use_fst = true))]
    fn itn_normalize_sentence_lang(
        py: Python<'_>,
        text: &str,
        lang: &str,
        use_fst: bool,
    ) -> String {
        py.detach(|| {
            if use_fst {
                crate::normalize_sentence_lang(text, lang)
            } else {
                crate::normalize_sentence_lang_rule_based(text, lang)
            }
        })
    }

    /// Convert a written sentence to spoken form using text normalization (TN).
    ///
    /// Set `use_fst` to `False` to use the rule-based engine for this call.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en", use_fst = true))]
    fn tn_normalize_sentence_lang(
        py: Python<'_>,
        text: &str,
        lang: &str,
        use_fst: bool,
    ) -> String {
        py.detach(|| {
            if use_fst {
                crate::tn_normalize_sentence_lang(text, lang)
            } else {
                crate::tn_normalize_sentence_lang_rule_based(text, lang)
            }
        })
    }

    #[pymodule_init]
    fn add_itn_alias(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add(
            "normalize_sentence_lang",
            module.getattr("itn_normalize_sentence_lang")?,
        )
    }
}
