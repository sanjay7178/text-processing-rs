from text_processing_rs import (
    itn_normalize_sentence_lang,
    normalize_sentence_lang,
    tn_normalize_sentence_lang,
)

assert normalize_sentence_lang is itn_normalize_sentence_lang
assert itn_normalize_sentence_lang("I have twenty one apples") == "I have 21 apples"
assert itn_normalize_sentence_lang(
    "Call me at five five five one two three four five six seven."
) == "Call me at 555-123-4567."
assert itn_normalize_sentence_lang("बीस रुपये", lang="hi") == "₹२०"
assert tn_normalize_sentence_lang("I paid $5 for 23 items") == (
    "I paid five dollars for twenty three items"
)
assert tn_normalize_sentence_lang("123", lang="it") == "cento ventitre"
