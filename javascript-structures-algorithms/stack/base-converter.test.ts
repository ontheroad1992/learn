import { decimalToBinary, baseConverter } from "./base-converter"

describe("base converter", () => {

  it("test the decimalToBinary method", () => {
    const binaryStr = decimalToBinary(4);
    expect(binaryStr).toBe('100')
  })

  it("test the baseConverter method, 2", () => {
    expect(baseConverter(4, 2)).toBe('100')
  })

  it("test the baseConverter method, 8", () => {
    expect(baseConverter(16, 8)).toBe('20')
  })


  it("test the baseConverter method, 16", () => {
    expect(baseConverter(100345, 16)).toBe('187F9')
  })
})
