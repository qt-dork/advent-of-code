import { zip } from "jsr:@std/collections/zip";
import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  const lists = input.trimEnd().split("\n").map((x) =>
    x.split("   ").map(Number)
  );
  const listA = lists.map((x) => {
    const [num, _] = x;
    return num;
  });
  const listB = lists.map((x) => {
    const [_, ber] = x;
    return ber;
  });
  return {
    a: listA,
    b: listB,
  };
}

function part1(input: string): number {
  const items = parse(input);
  const sortedItems = {
    a: items.a.toSorted((a, b) => a - b),
    b: items.b.toSorted((a, b) => a - b),
  };

  const zipped = zip(sortedItems.a, sortedItems.b).map(([a, b]) =>
    Math.abs(a - b)
  ).reduce((accum, cur) => accum + cur);
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

  const scores = items.a.map((x) => {
    const matches = items.b.filter((y) => x === y);
    const sum = matches.reduce((acc, _) => (acc + 1), 0);
    return x * sum;
  });
  const score = scores.reduce((accum, cur) => accum + cur); // I need to make a sum function
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
