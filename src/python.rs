/// Sentence-level inverse text normalization (ITN) and text normalization (TN).
#[pyo3::pymodule]
mod text_processing_rs {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

    use crate::DEFAULT_MAX_SPAN_TOKENS;

    fn require_fst(output: Option<String>) -> PyResult<String> {
        output.ok_or_else(|| {
            PyRuntimeError::new_err(
                "This Python extension was built without the fst-engine feature",
            )
        })
    }

    /// Convert a spoken sentence to written form using inverse text normalization (ITN).
    ///
    /// Language codes are en, de, es, fr, hi, ja, and zh. An unrecognized code
    /// falls back to English. The default uses the rule-based taggers. Pass
    /// use_fst_engine=True to run the compiled NeMo grammar for this call.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en", use_fst_engine = false))]
    fn itn_normalize_sentence_lang(
        py: Python<'_>,
        text: &str,
        lang: &str,
        use_fst_engine: bool,
    ) -> PyResult<String> {
        py.detach(|| {
            if use_fst_engine {
                return require_fst(crate::fst_itn(text, lang));
            }
            Ok(crate::rule_based_itn_sentence(
                text,
                lang,
                DEFAULT_MAX_SPAN_TOKENS,
            ))
        })
    }

    /// Convert a written sentence to spoken form using text normalization (TN).
    ///
    /// The default uses the rule-based taggers. Pass use_fst_engine=True to
    /// run the compiled NeMo grammar for this call, including Italian TN.
    #[pyfunction]
    #[pyo3(signature = (text, lang = "en", use_fst_engine = false))]
    fn tn_normalize_sentence_lang(
        py: Python<'_>,
        text: &str,
        lang: &str,
        use_fst_engine: bool,
    ) -> PyResult<String> {
        py.detach(|| {
            if use_fst_engine {
                return require_fst(crate::fst_tn(text, lang));
            }
            Ok(crate::rule_based_tn_sentence(
                text,
                lang,
                DEFAULT_MAX_SPAN_TOKENS,
            ))
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
