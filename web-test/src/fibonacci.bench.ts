import { bench, describe } from "vitest";
import { fibonacci } from "./fibonacci";

describe("fibonacci", () => {
  bench("fibo 10", () => {
    fibonacci(10);
  });

  bench("fibo 15", () => {
    fibonacci(15);
  });
});
