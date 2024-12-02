import { sumOf, unzip, zip } from "jsr:@std/collections";
import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  const lists = input.trimEnd().split("\n").map((x) =>
    x.split("   ").map(Number) as [number, number] // can only have two numbers in input
  );
  const unpaired = unzip(lists);
  return unpaired;
}

function part1(input: string): number {
  const items = parse(input);
  const sorted = [
    items[0].toSorted((a, b) => a - b),
    items[1].toSorted((a, b) => a - b),
  ];

  const zipped = zip(sorted[0], sorted[1]).reduce(
    (accum, [a, b]) => (accum + Math.abs(a - b)),
    0,
  );
  // throw new Error("TODO");
  console.log(zipped);
  return zipped;
}

function part2(input: string): number {
  const items = parse(input);
  // throw new Error("TODO");
  // const sortedItems = {
  //   a: items.a.toSorted((a, b) => a - b),
  //   b: items.b.toSorted((a, b) => a - b),
  // };

  const scores = items[0].map((x) => {
    const sum = items[1].filter((y) => x === y).length;
    return x * sum;
  });
  const score = sumOf(scores, (x) => x);
  // throw new Error("TODO");
  return score;
}

if (import.meta.main) {
  runPart(2024, 1, 1, part1);
  runPart(2024, 1, 2, part2);
}

const TEST_INPUT = `\
3   4
4   3
2   5
1   3
3   9
3   3
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 11);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 31);
});
