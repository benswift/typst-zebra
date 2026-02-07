#import "../src/lib.typ" as zebra

#set page(
  width: auto,
  height: auto,
  fill: luma(90%),
  margin: 1em,
)

#for block in read("../README.md").split("## Native Rendering").at(0).split("```") {
  if not block.starts-with("typ") {
    continue
  }
  block = block.slice(3)
  block = block.split("\n").filter(line => not line.starts-with("#import")).join("\n")
  block = block.trim()

  raw(block, block: true, lang: "typ")
  eval(block, mode: "markup", scope: dictionary(zebra))
}
