const items: WeakMap<StackObject<any>, any[]> = new WeakMap()

export class StackObject<T> {
  #count: number = 0;

  constructor() {
    items.set(this, [])
  }

  push(element: T) {
    const s = items.get(this)
    s?.push(element)
    this.#count += 1;
  }

  size() {
    return this.#count
  }

  isEmpty() {
    return this.#count === 0
  }

  pop() {
    if (this.isEmpty()) {
      return undefined
    }

    this.#count -= 1;
    const s = items.get(this)

    return s?.pop()
  }

  peek(): any {
    if (this.isEmpty()) {
      return undefined
    }

    return items.get(this)?.[this.#count - 1]
  }

  clear() {
    items.set(this, [])
    this.#count = 0
  }

  toString(): string {
    if (this.isEmpty()) {
      return ''
    }

    const s = items.get(this)
    let objectString = s?.[0] + ''
    for (let i = 1; i < this.#count; i++) {
      objectString = objectString + ',' + s?.[i]
    }
    return objectString
  }
}
