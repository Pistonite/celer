import { ExecDoc } from "data/model";
import { getRelativeLocation } from "./getRelativeLocation";

describe("core/utils/getRelativeLocation", () => {
    const mockDoc = {
        route: [
            { lines: { length: 5 } },
            { lines: { length: 6 } },
            { lines: { length: 7 } },
        ],
    } as ExecDoc;

    it("should return the same location if delta is 0", () => {
        const loc = { section: 1, line: 2 };
        expect(getRelativeLocation(mockDoc, loc.section, loc.line, 0)).toEqual(loc);
    });

    it("should return previous location, same section", () => {
        expect(getRelativeLocation(mockDoc, 1, 2, -1)).toEqual({
            section: 1,
            line: 1,
        });
        expect(getRelativeLocation(mockDoc, 1, 2, -2)).toEqual({
            section: 1,
            line: 0,
        });
    });

    it("should return previous location, previous section", () => {
        expect(getRelativeLocation(mockDoc, 1, 2, -3)).toEqual({
            section: 0,
            line: 4,
        });
        expect(getRelativeLocation(mockDoc, 1, 2, -4)).toEqual({
            section: 0,
            line: 3,
        });
        expect(getRelativeLocation(mockDoc, 2, 1, -7)).toEqual({
            section: 1,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 2, 1, -8)).toEqual({
            section: 0,
            line: 4,
        });
    });

    it("should return previous location, boundary", () => {
        expect(getRelativeLocation(mockDoc, 0, 2, -2)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 0, 2, -3)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 1, 0, -5)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 1, 0, -6)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 2, 1, -12)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 2, 1, -13)).toEqual({
            section: 0,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 2, 1, -11)).toEqual({
            section: 0,
            line: 1,
        });
    });

    it("should return next location, same section", () => {
        expect(getRelativeLocation(mockDoc, 1, 2, 1)).toEqual({
            section: 1,
            line: 3,
        });
        expect(getRelativeLocation(mockDoc, 1, 2, 3)).toEqual({
            section: 1,
            line: 5,
        });
    });

    it("should return next location, next section", () => {
        expect(getRelativeLocation(mockDoc, 1, 2, 4)).toEqual({
            section: 2,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 1, 2, 5)).toEqual({
            section: 2,
            line: 1,
        });
        expect(getRelativeLocation(mockDoc, 0, 1, 10)).toEqual({
            section: 2,
            line: 0,
        });
        expect(getRelativeLocation(mockDoc, 0, 1, 11)).toEqual({
            section: 2,
            line: 1,
        });
    });

    it("should return next location, boundary", () => {
        expect(getRelativeLocation(mockDoc, 0, 2, 14)).toEqual({
            section: 2,
            line: 5,
        });
        expect(getRelativeLocation(mockDoc, 0, 2, 15)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 0, 2, 16)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 1, 4, 7)).toEqual({
            section: 2,
            line: 5,
        });
        expect(getRelativeLocation(mockDoc, 1, 4, 8)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 1, 4, 9)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 2, 5, 1)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 2, 6, 7)).toEqual({
            section: 2,
            line: 6,
        });
        expect(getRelativeLocation(mockDoc, 2, 5, 8)).toEqual({
            section: 2,
            line: 6,
        });
    });

});
