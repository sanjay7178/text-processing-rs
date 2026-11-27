from text_processing_rs import (
    itn_normalize_sentence_lang,
    normalize_sentence_lang,
    tn_normalize_sentence_lang,
)

assert normalize_sentence_lang is itn_normalize_sentence_lang
assert itn_normalize_sentence_lang("I have twenty one apples") == "I have 21 apples"
assert tn_normalize_sentence_lang("I paid $5 for 23 items") == (
    "I paid five dollars for twenty three items"
)

phone = "Call me at five five five one two three four five six seven."
assert normalize_sentence_lang(phone, use_fst_engine=True) == "Call me at 555-123-4567."
assert itn_normalize_sentence_lang(phone) == "Call me at 5551234567."
assert itn_normalize_sentence_lang(phone, "en", False) == "Call me at 5551234567."

hindi = "मेरे पास पच्चीस किताबें हैं।"
assert itn_normalize_sentence_lang(hindi, lang="hi") == "मेरे पास २५ किताबें हैं।"
assert itn_normalize_sentence_lang(hindi, "hi", use_fst_engine=True) == hindi

assert tn_normalize_sentence_lang("Chapter IV", use_fst_engine=True) == "Chapter IV"
assert tn_normalize_sentence_lang("Chapter IV", use_fst_engine=False) == "Chapter four"
assert tn_normalize_sentence_lang("123", "it", True) == "cento ventitre"
assert tn_normalize_sentence_lang("123", "it") == "one hundred and twenty three"
