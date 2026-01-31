#import "../src/lib.typ" as zebra
#import "@preview/cades:0.3.1"
#import "@preview/codetastic:0.2.2"
#import "@preview/rustycure:0.2.0"
#import "@preview/tiaoma:0.3.0"

#set page(width: auto, height: auto, margin: 1em)
#set text(font: "DejaVu Sans Mono", size: 10pt)

#let data = "0"
#let width = 5em

#grid(
  columns: (auto, auto),
  gutter: 1em,
  [cades], cades.qr-code(data, width: width),
  [codetastic], codetastic.qrcode(data, quiet-zone: 0, width: width),
  [rustycure], rustycure.qr-code(data, quiet-zone: false, width: width),
  [tiaoma], tiaoma.qrcode(data, width: width),
  [zebra], zebra.qrcode(data, width: width),
)
