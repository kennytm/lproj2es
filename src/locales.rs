//! Collection of well-known locale IDs.
//!
//! This module provides a single function `locale_id` that maps an `*.lproj` file name to a canonical locale ID.
//! Aliases to the same locale, e.g. `English.lproj`, `en.lproj` and `en_US.lproj` are all mapped to the same output,
//! `en_US`.
//!
//! This combination simplifies searching, because the user will only need to look for the `en_US` key, instead of all
//! three variants.
//!
//! The default country of a language is referenced from these two links:
//!
//! * <https://lh.2xlibre.net/locales/>
//! * <http://stackoverflow.com/questions/3040677/locale-codes-for-iphone-lproj-folders>

use std::collections::HashMap;

lazy_static! {
    // TODO: Switch to phf once it can run on stable without build.rs.
    static ref LOCALE_MAP: HashMap<&'static str, &'static str> = hashmap!{
        "aa" => "aa_ET",
        "af" => "af_ZA", "Afrikaans" => "af_ZA",
        "ak" => "ak_GH",
        "am" => "am_ET", "Amharic" => "am_ET",
        "an" => "an_ES",
        "ar" => "ar_SA", "Arabic" => "ar_SA",
        "as" => "as_IN", "Assamese" => "as_IN",
        "ay" => "ay_PE", "Aymara" => "ay_PE",
        "az" => "az_AZ", "Azerbaijani" => "az_AZ",

        "be" => "be_BY", "Byelorussian" => "be_BY",
        "bg" => "bg_BG", "Bulgarian" => "bg_BG",
        "bi" => "bi_TV",
        "bm" => "bm_ML",
        "bn" => "bn_BD", "Bengali" => "bn_BD",
        "bo" => "bo_CN", "Tibetan" => "bo_CN",
        "br" => "br_FR", "Breton" => "br_FR",
        "bs" => "bs_BA",

        "ca" => "ca_ES", "Catalan" => "ca_ES",
        "ce" => "ce_RU",
        "cs" => "cs_CZ", "Czech" => "cs_CZ",
        "cv" => "cv_RU",
        "cy" => "cy_GB", "Welsh" => "cy_GB",

        "da" => "da_DK", "Danish" => "da_DK",
        "de" => "de_DE", "German" => "de_DE",
        "dv" => "dv_MV",
        "dz" => "dz_BT", "Dzongkha" => "dz_BT",

        "el" => "el_GR", "Greek" => "el_GR",
        "en" => "en_US", "English" => "en_US",
        "eo" => "eo_001", "Esperanto" => "eo_001",
        "es" => "es_ES", "Spanish" => "es_ES",
        "et" => "et_EE", "Estonian" => "et_EE",
        "eu" => "eu_ES", "Basque" => "eu_ES",

        "fa" => "fa_IR", "Farsi" => "fa_IR",
        "ff" => "ff_SN",
        "fi" => "fi_FI", "Finnish" => "fi_FI",
        "fo" => "fo_FO", "Faroese" => "fo_FO",
        "fr" => "fr_FR", "French" => "fr_FR",
        "fy" => "fy_DE",

        "ga" => "ga_IE", "Irish" => "ga_IE",
        "gd" => "gd_GB", "Scottish" => "gd_GB",
        "gl" => "gl_ES", "Galician" => "gl_ES",
        "gn" => "gn_PY", "Guarani" => "gn_PY",
        "gu" => "gu_IN", "Gujarati" => "gu_IN",
        "gv" => "gv_GB", "Manx" => "gv_GB",

        "ha" => "ha_NG",
        "he" => "he_IL", "Hebrew" => "he_IL",
        "hi" => "hi_IN", "Hindi" => "hi_IN",
        "hr" => "hr_HR", "Croatian" => "hr_HR",
        "ht" => "ht_HT",
        "hu" => "hu_HU", "Hungarian" => "hu_HU",
        "hy" => "hy_AM", "Armenian" => "hy_AM",

        "ia" => "ia_FR",
        "id" => "id_ID", "Indonesian" => "id_ID",
        "ig" => "ig_NG",
        "ik" => "ik_CA",
        "is" => "is_IS", "Icelandic" => "is_IS",
        "it" => "it_IT", "Italian" => "it_IT",
        "iu" => "iu_CA", "Inuktitut" => "iu_CA",

        "ja" => "ja_JP", "Japanese" => "ja_JP",
        "jv" => "jv_ID", "Javanese" => "jv_ID",

        "ka" => "ka_GE", "Georgian" => "ka_GE",
        "kk" => "kk_KZ", "Kazakh" => "kk_KZ",
        "kl" => "kl_GL", "Greenlandic" => "kl_GL",
        "km" => "km_KH", "Khmer" => "km_KH",
        "kn" => "kn_IN", "Kannada" => "kn_IN",
        "ko" => "ko_KR", "Korean" => "ko_KR",
        "ks" => "ks_IN", "Kashmiri" => "ks_IN",
        "ku" => "ku_TR", "Kurdish" => "ku_TR",
        "kw" => "kw_GB",
        "ky" => "ky_KG", "Kirghiz" => "ky_KG",

        "la" => "la_IT", "Latin" => "la_IT",
        "lb" => "lb_LU",
        "lg" => "lg_UG",
        "li" => "li_NL",
        "ln" => "ln_CD",
        "lo" => "lo_LA", "Lao" => "lo_LA",
        "lt" => "lt_LT", "Lithuanian" => "lt_LT",
        "lv" => "lv_LV", "Latvian" => "lv_LV",

        "mg" => "mg_MG", "Malagasy" => "mg_MG",
        "mh" => "mh_MH",
        "mi" => "mi_NZ",
        "mk" => "mk_MK", "Macedonian" => "mk_MK",
        "ml" => "ml_IN", "Malayalam" => "ml_IN",
        "mn" => "mn_MN", "Mongolian" => "mn_MN",
        "mo" => "mo_MD", "Moldavian" => "mo_MD",
        "mr" => "mr_IN", "Marathi" => "mr_IN",
        "ms" => "ms_MY", "Malay" => "ms_MY",
        "mt" => "mt_MT", "Maltese" => "mt_MT",
        "my" => "my_MM", "Burmese" => "my_MM",

        "nb" => "nb_NO", "Norwegian" => "nb_NO",
        "ne" => "ne_NP", "Nepali" => "ne_NP",
        "nl" => "nl_NL", "Dutch" => "nl_NL",
        "nn" => "nn_NO", "Nynorsk" => "nn_NO",
        "nr" => "nr_ZA",
        "ny" => "ny_MW", "Nyanja" => "ny_MW",
        "oc" => "oc_FR",
        "om" => "om_ET", "Oromo" => "om_ET",
        "or" => "or_IN", "Oriya" => "or_IN",
        "os" => "os_RU",

        "pa" => "pa_IN", "Punjabi" => "pa_IN",
        "pl" => "pl_PL", "Polish" => "pl_PL",
        "ps" => "ps_AF", "Pashto" => "ps_AF",
        "pt" => "pt_BR", "Portuguese" => "pt_BR",

        "qu" => "qu_PE", "Quechua" => "qu_PE",

        "rn" => "rn_BI", "Rundi" => "rn_BI",
        "ro" => "ro_RO", "Romanian" => "ro_RO",
        "ru" => "ru_RU", "Russian" => "ru_RU",
        "rw" => "rw_RW", "Kinyarwanda" => "rw_RW",

        "sa" => "sa_IN", "Sanskrit" => "sa_IN",
        "sc" => "sc_IT",
        "sd" => "sd_IN", "Sindhi" => "sd_IN",
        "se" => "se_NO", "Sami" => "se_NO",
        "si" => "si_LK", "Sinhalese" => "si_LK",
        "sk" => "sk_SK", "Slovak" => "sk_SK",
        "sl" => "sl_SI", "Slovenian" => "sl_SI",
        "so" => "so_SO", "Somali" => "so_SO",
        "sq" => "sq_AL", "Albanian" => "sq_AL",
        "sr" => "sr_CS", "Serbian" => "sr_CS",
        "ss" => "ss_ZA",
        "st" => "st_ZA",
        "su" => "su_ID", "Sundanese" => "su_ID",
        "sv" => "sv_SE", "Swedish" => "sv_SE",
        "sw" => "sw_TZ", "Swahili" => "sw_TZ",

        "ta" => "ta_IN", "Tamil" => "ta_IN",
        "te" => "te_IN", "Telugu" => "te_IN",
        "tg" => "tg_TJ", "Tajiki" => "tg_TJ",
        "th" => "th_TH", "Thai" => "th_TH",
        "ti" => "ti_ER", "Tigrinya" => "ti_ER",
        "tk" => "tk_TM", "Turkmen" => "tk_TM",
        "tl" => "tl_PH", "Tagalog" => "tl_PH",
        "tn" => "tn_ZA",
        "to" => "to_TO", "Tongan" => "to_TO",
        "tr" => "tr_TR", "Turkish" => "tr_TR",
        "ts" => "ts_ZA",
        "tt" => "tt_RU", "Tatar" => "tt_RU",

        "ug" => "ug_CN", "Uighur" => "ug_CN",
        "uk" => "uk_UA", "Ukrainian" => "uk_UA",
        "ur" => "ur_PK", "Urdu" => "ur_PK",
        "uz" => "uz_UZ", "Uzbek" => "uz_UZ",

        "ve" => "ve_ZA",
        "vi" => "vi_VN", "Vietnamese" => "vi_VN",

        "wa" => "wa_BE",
        "wo" => "wo_SN",

        "xh" => "xh_ZA",

        "yi" => "yi_US", "Yiddish" => "yi_US",
        "yo" => "yo_NG",

        "zh" => "zh_CN",
        "zu" => "zu_ZA",



        "agr" => "agr_PE",
        "anp" => "anp_IN",
        "ast" => "ast_ES",
        "ayc" => "ayc_PE",

        "bem" => "bem_ZM",
        "ber" => "ber_DZ",
        "bhb" => "bhb_IN",
        "bho" => "bho_IN",
        "brx" => "brx_IN",
        "byn" => "byn_ER",

        "chr" => "chr_US",
        "crh" => "crh_UA",
        "csb" => "csb_PL",

        "doi" => "doi_IN",

        "fil" => "fil_PH",
        "fur" => "fur_IT",

        "gez" => "gez_ER",
        "grc" => "grc_GR",

        "hak" => "hak_TW",
        "hne" => "hne_IN",
        "hsb" => "hsb_DE",
        "hus" => "hus_MX",

        "kab" => "kab_DZ",
        "kok" => "kok_IN",

        "lij" => "lij_IT",
        "lzh" => "lzh_TW",

        "mag" => "mag_IN",
        "mai" => "mai_IN",
        "mhr" => "mhr_RU",
        "miq" => "miq_NI",
        "myv" => "myv_RU",

        "nah" => "nah_MX",
        "nan" => "nan_TW",
        "nds" => "nds_DE",
        "nhn" => "nhn_MX",
        "niu" => "niu_NU",
        "nso" => "nso_ZA",

        "pap" => "pap_AW",

        "quy" => "quy_PE",
        "quz" => "quz_PE",

        "raj" => "raj_IN",

        "sat" => "sat_IN",
        "sgs" => "sgs_LT",
        "shs" => "shs_CA",
        "sid" => "sid_ET",
        "son" => "son_ML",
        "szl" => "szl_PL",

        "tcy" => "tcy_IN",
        "the" => "the_NP",
        "tig" => "tig_ER",

        "unm" => "unm_US",

        "wae" => "wae_CH",
        "wal" => "wal_ET",

        "yue" => "yue_HK",



        "zh-Hant" => "zh_TW",
        "zh-Hans" => "zh_CN",

        "cmn" => "zh_CN",
        "cmn_TW" => "zh_TW",
        "cmn_CN" => "zh_CN",

        "no" => "nb_NO",    // Sometimes the misspelled language code "no" is used instead of "nb".
        "sr_RS" => "sr_CS", // Apple uses the outdated "CS" country code instead of "RS".
    };
}

/// Gets the locale ID of an `*.lproj` folder. The locale ID is always of the form like `"en_US"`.
pub fn locale_id(lproj_name: &str) -> &str {
    debug_assert!(lproj_name.ends_with(".lproj"));
    let name = &lproj_name[.. lproj_name.len()-6];
    LOCALE_MAP.get(name).unwrap_or(&name)
}

#[test]
fn test_locale_id() {
    assert_eq!(locale_id("en.lproj"), "en_US");
    assert_eq!(locale_id("French.lproj"), "fr_FR");
    assert_eq!(locale_id("es_419.lproj"), "es_419");
    assert_eq!(locale_id("unknown.lproj"), "unknown");
}

/*

Copyright 2017 kennytm

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/