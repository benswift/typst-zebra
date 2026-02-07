#import "../src/generic.typ": encode

#set page(
  width: auto,
  height: auto,
  margin: 0pt,
  fill: none,
)

#let pixel-segments = {
  // @typstyle off
  let pixels = (0, 2, 4, 6, 8, 10, 11, 12, 14, 15, 16, 17, 19, 20, 22, 23, 24, 25, 26, 27, 30, 31, 33, 35, 37, 39, 40, 44, 45, 46, 48, 50, 52, 54, 56, 57, 58, 59, 60, 64, 66, 67, 68, 70, 71, 75, 78, 79, 80, 82, 83, 84, 88, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99)
  let segments = ()
  let (x, y) = (0, 0)
  for position in pixels {
    let dx = calc.rem(position, 10) - x
    let dy = calc.quo(position, 10) - y
    (x, y) = (x + dx, y + dy)
    segments.push(curve.move((dx * 1pt, dy * 1pt), relative: true))
    segments.push(curve.line((1pt, 0pt), relative: true))
    segments.push(curve.line((0pt, 1pt), relative: true))
    segments.push(curve.line((-1pt, 0pt), relative: true))
    segments.push(curve.close())
  }
  segments
}

#let optimized-segments = encode(
  "20912",
  "datamatrix",
  (:),
).segments

#let arrows(segments) = {
  let offset = 0.15pt
  let arrow-size = 0.2pt

  let arrows = ()
  let (x, y) = (0pt, 0pt)
  let (move-x, move-y) = (0pt, 0pt)
  for segment in segments {
    let (dx, dy) = if segment.func() == curve.move {
      x += segment.start.at(0).length
      y += segment.start.at(1).length
      (move-x, move-y) = (x, y)
      continue
    } else if segment.func() == curve.line {
      let dx = segment.end.at(0).length
      let dy = segment.end.at(1).length
      (dx, dy)
    } else if segment.func() == curve.close {
      (move-x - x, move-y - y)
    } else {
      panic()
    }
    let (sx, sy) = (dx, dy).map(d => calc.clamp(d / 1pt, -1, 1))
    arrows.push(curve.move((x + sx * offset, y + sy * offset)))
    (x, y) = (x + dx, y + dy)
    arrows.push(curve.line((x - sx * offset, y - sy * offset)))
    arrows.push(curve.move(
      ((-sx + sy) * arrow-size, (-sy + -sx) * arrow-size),
      relative: true,
    ))
    arrows.push(curve.line(
      ((sx + -sy) * arrow-size, (sy + sx) * arrow-size),
      relative: true,
    ))
    arrows.push(curve.line(
      ((-sx + -sy) * arrow-size, (-sy + sx) * arrow-size),
      relative: true,
    ))
  }
  arrows
}

#let show-segments(segments) = block(width: 23pt, height: 12pt, {
  set place(dx: 1pt, dy: 1pt)
  place(
    curve(
      fill: luma(90%),
      fill-rule: "even-odd",
      ..optimized-segments,
    ),
  )
  place(
    curve(
      stroke: 0.1pt,
      ..arrows(segments),
    ),
  )
  place(
    dx: 12pt,
    curve(
      fill: black,
      fill-rule: "even-odd",
      stroke: 0.1pt + luma(50%),
      ..segments,
    ),
  )
})

#scale(1000%, origin: top + left, reflow: true, {
  stack(
    dir: ltr,
    spacing: 1pt,
    ..(
      show-segments(pixel-segments),
      show-segments(optimized-segments),
    ).map(content => block(content, fill: white, radius: 0.5pt)),
  )
})
