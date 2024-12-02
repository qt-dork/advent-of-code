import { slidingWindows } from "jsr:@std/collections";
import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  const lists = input.trimEnd().split("\n").map((x) => x.split(" ").map(Number) // can only have two numbers in input
  );
  return lists;
}

function test(item: number[]): boolean {
  const itemWindowThree = slidingWindows(item, 3);
  const isOnlyUpOrDown = itemWindowThree.some(([a, b, c]) =>
    (a - b > 0) !== (b - c > 0) || (a - b == 0) || (b - c == 0)
  );
  const itemWindowTwo = slidingWindows(item, 2);
  const isInRange = itemWindowTwo.some(([a, b]) => Math.abs(a - b) > 3);

  return !(isOnlyUpOrDown || isInRange);
}

function part1(input: string): number {
  const items = parse(input);
  const result = items.filter((item) => {
    return test(item);
  }).length;
  // throw new Error("TODO");
  return result;
}

function part2(input: string): number {
  const items = parse(input);
  const result = items.filter((item) => {
    let arrItems: number[][] = [];
    item.forEach((_, i) => {
      arrItems.push(item.toSpliced(i, 1));
    });
    return arrItems.some(test);
  });

  return result.length;
  //   let output = false;
  //   let itemCopy = item;
  //   const direction = slidingWindows(item, 2).reduce(
  //     (accum, [a, b]) => (accum + (a - b > 0 ? 1 : -1)),
  //     0,
  //   ) > 0;
  //   for (let i = 0; i < itemCopy.length; i++) {
  //     const x = itemCopy[i];
  //     const y = itemCopy[i + 1];
  //     if (
  //       (i + 1 < itemCopy.length && (x - y > 0) !== direction)
  //     ) {
  //       if (!output) {
  //         itemCopy = itemCopy.toSpliced(i, 1);
  //         output = true;
  //         i = 0;
  //         continue;
  //       } else {
  //         return false;
  //       }
  //     }
  //     if (
  //       i + 1 < itemCopy.length &&
  //       ((Math.abs(x - y) > 3) || Math.abs(x - y) === 0)
  //     ) {
  //       if (!output) {
  //         itemCopy = itemCopy.toSpliced(i + 1, 1);
  //         output = true;
  //         i = 0;
  //         continue;
  //       } else {
  //         return false;
  //       }
  //     }
  //   }
  //   return true;
  // }).length;
  // // throw new Error("TODO");
  // return result;
  // // throw new Error("TODO");
}

if (import.meta.main) {
  runPart(2024, 2, 1, part1);
  runPart(2024, 2, 2, part2);
}

const TEST_INPUT = `\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 2);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 4);
});
