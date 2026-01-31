#import "generic.typ": _generic

#let datamatrix(
  data,
  options: (:),
  ..common-args,
) = _generic(
  data,
  "datamatrix",
  options: options,
  ..common-args,
)

#let qrcode(
  data,
  options: (:),
  ..common-args,
) = _generic(
  data,
  "qrcode",
  options: options,
  ..common-args,
  default-quiet-zone: 4,
)
