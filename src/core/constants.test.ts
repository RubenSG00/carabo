import * as constants from "./constants";
import expected from "../../fixtures/constants.json";

describe("constants (contract with fixtures/constants.json, 08 §7)", () => {
  it("exports every constant of 00 §4 with its value, and nothing else but REPOSITORY_URL", () => {
    const exported = Object.fromEntries(
      Object.entries(constants).filter(([name]) => name !== "REPOSITORY_URL"),
    );
    expect(exported).toEqual(expected);
  });

  it("exports REPOSITORY_URL as an HTTPS address or as the empty string (07 §7)", () => {
    expect(constants.REPOSITORY_URL).toMatch(/^(https:\/\/\S+)?$/);
  });
});
