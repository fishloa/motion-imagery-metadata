# MISB standards: full-document Markdown

Source: `specs/*.pdf`, one `PROVENANCE.md` each. Converted with `pdf2md convert
--engine hybrid --max-pages 0 --report` (2026-11-23), then proofread page by page
against `pdftotext -layout` and page renders: tables rebuilt to one row per
logical row, hex dumps as fenced blocks in printed order, page furniture removed.
Wording is the PDF's, including its typos.

"Verification" is pdf2md's token diff (hex, bit and digit runs) against the PDF
text layer, taken on the raw conversion. "Order-only" means every flagged token
is present on both sides of the same page in a different order; the flagged
pages were the first ones proofread.

Known limits of the proofread text: labels inside image-only figures are kept
as fenced blocks in approximate reading order (or dropped where meaningless
without the drawing), merged table headers and column spans are flattened, a
nested table (ST 0903.6 Table 21) is split out, and equations are linearised or
written as LaTeX. Captions, values and prose are as printed.

Verified table transcriptions the test suite reads are in `specs-verified/misb/`.

| stem | pages | verification |
|---|---|---|
| misb_st0102.12_security_local_set | 18 | clean |
| misb_st0107.5_2021_klv_metadata_in_motion_imagery | 8 | clean |
| misb_st0601.19_2023_uas_datalink_local_set | 226 | 10 order-only warnings (pages 4, 7), no value changed or dropped |
| misb_st0603.5_2017_misp_time_system_and_timestamps | 10 | clean |
| misb_st0604.6_2017_timestamps_class1_class2_motion_imagery | 11 | clean |
| misb_st0806.4_2014_remote_video_terminal_metadata_set | 13 | clean |
| misb_st0902.5_2015_motion_imagery_sensor_minimum_metadata_set | 13 | 32 order-only warnings (pages 8, 10), no value changed or dropped |
| misb_st0902.8_2018_motion_imagery_sensor_minimum_metadata_set | 17 | 48 order-only warnings (pages 11, 12, 13), no value changed or dropped |
| misb_st0903.6_vmti_local_set | 72 | 2 order-only warnings (page 22), no value changed or dropped |
| misb_st1002.2_2016_range_motion_imagery | 22 | clean |
| misb_st1201.5_2021_floating_point_to_integer_mapping | 25 | clean |
| misb_st1204.2_2019_miis_core_identifier | 38 | 2 order-only warnings (page 17), no value changed or dropped |
| misb_st1206.1_2017_sar_motion_imagery_metadata | 23 | clean |
| misb_st1303.2_2020_multi_dimensional_array_pack | 24 | 8 order-only warnings (pages 17, 18), no value changed or dropped |
| misb_st1601.2_2023_geo_registration_local_set | 11 | clean |
| misb_st1602.2_2023_composite_imaging_local_set | 18 | clean |
| misb_st1607_2016_amend_segment_klv_metadata | 7 | clean |
