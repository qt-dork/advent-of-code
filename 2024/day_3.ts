import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

interface Operation {
  operation: "mul" | "do" | "dont";
  operands?: number[];
}

function parse(input: string): Operation[] {
  return input.trimEnd().split("\n").map((line) =>
    line.matchAll(/mul\((\d+),(\d+)\)|do\(\)|don't\(\)/g).map((
      [match, a, b, ..._],
    ) => {
      if (match.includes("don't")) {
        return {
          operation: "dont",
        } as Operation;
      } else if (match.includes("do")) {
        return {
          operation: "do",
        } as Operation;
      } else {
        return {
          operation: "mul",
          operands: [Number(a), Number(b)],
        } as Operation;
      }
    })
  ).flatMap((iteratorObject) => iteratorObject.toArray());
}

function part1(input: string): number {
  const items = parse(input);
  // throw new Error("TODO");
  const output = items.filter((item) => item.operation === "mul").map((item) =>
    item.operands!.reduce((accum, cur) => accum * cur)
  ).reduce((accum, cur) => accum + cur);
  // return items.map((item) => {item.})
  return output;
}

function part2(input: string): number {
  const items = parse(input);
  let on = true;
  let tally = 0;
  for (const item of items) {
    if (item.operation == "do") {
      on = true;
    } else if (item.operation == "dont") {
      on = false;
    }

    if (on && item.operation == "mul") {
      tally += item.operands!.reduce((accum, cur) => accum * cur);
    }
  }
  return tally;
}

if (import.meta.main) {
  runPart(2024, 3, 1, part1);
  runPart(2024, 3, 2, part2);
}

const TEST_INPUT_PART_1 = `\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
`;

const TEST_INPUT_PART_2 = `\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT_PART_1), 161);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT_PART_2), 48);
});
