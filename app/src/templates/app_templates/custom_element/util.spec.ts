// This file is just to get you started with your unit testing. Feel free to delete it.

import { utilityFunction } from "./util";
import { expect } from "@jest/globals";

describe("Utility function", () => {
  it("should always return true", () => {
    expect(utilityFunction()).toBe(true);
  });
});
