from text_processing_rs import (
    itn_normalize_sentence_lang,
    normalize_sentence_lang,
    tn_normalize_sentence_lang,
)

assert normalize_sentence_lang is itn_normalize_sentence_lang
assert itn_normalize_sentence_lang("I have twenty one apples") == "I have 21 apples"
assert itn_normalize_sentence_lang("मेरे पास पच्चीस किताबें हैं।", lang="hi") == (
    "मेरे पास २५ किताबें हैं।"
)
assert tn_normalize_sentence_lang("I paid $5 for 23 items") == (
    "I paid five dollars for twenty three items"
)
