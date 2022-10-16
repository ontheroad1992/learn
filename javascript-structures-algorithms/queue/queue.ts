export class Queue<T extends any> {
  #count = 0;
  #lowestCount = 0;
  #items: { [key: number]: T } = {}

  enqueue(element: T) {
    this.#items[this.#count] = element
    this.#count += 1;
  }

  dequeue() {
    if (this.isEmpty()) {
      return undefined
    }

    const result = this.#items[this.#lowestCount]
    delete this.#items[this.#lowestCount]
    this.#lowestCount += 1;

    return result
  }

  peek(): T | undefined {
    if (this.isEmpty()) {
      return undefined
    }

    return this.#items[this.#count - 1]
  }

  size() {
    return this.#count - this.#lowestCount
  }

  isEmpty() {
    return this.size() === 0;
  }

  clear() {
    this.#count = 0;
    this.#lowestCount = 0;
    this.#items = {};
  }

  toString() {
    if (this.isEmpty()) {
      return ''
    }

    let str = this.#items[this.#lowestCount] + ''
    for (let i = this.#lowestCount + 1; i < this.#count; i++) {
      str = str + ',' + this.#items[i]
    }

    return str;
  }
}
