
export class StackObject<T> {
  count: number = 0;
  items: { [key: number]: T } = {};

  push(element: T) {
    this.items[this.count] = element
    this.count += 1;
  }

  size() {
    return this.count
  }

  isEmpty() {
    return this.count === 0
  }

  pop() {
    if (this.isEmpty()) {
      return undefined
    }

    this.count -= 1;
    const result = this.items[this.count]
    delete this.items[this.count]
    return result
  }

  peek() {
    if (this.isEmpty()) {
      return undefined
    }

    return this.items[this.count -1]
  }

  clear() {
    this.items = {}
    this.count = 0
  }

  toString() {
    if (this.isEmpty()) {
      return ''
    }

    let objectString = this.items[0] + ''
    for (let i = 1; i < this.count; i++) {
      objectString = objectString + ',' + this.items[i]
    }
    return objectString
  }
}
