import { StackArray } from './stack-array'
import { StackObject } from './stack-object'

export function decimalToBinary(decNumber: number) {
  const stack = new StackArray();
  let number = decNumber;
  let rem: number;
  let binaryString = '';

  while (number > 0) {
    rem = number % 2
    stack.push(rem)
    number = Math.floor(number / 2)
  }

  while(!stack.isEmpty()) {
    binaryString += stack.pop()
  }

  return binaryString;
}

export function baseConverter(decNumber: number, base: number) {
  const stack = new StackObject<number>()
  const digits = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  let number = decNumber;
  let rem: number;
  let baseString = '';

  if (!(base >=2 && base <= 36)) {
    return;
  }

  while(number > 0) {
    rem = Math.floor(number % base);
    stack.push(rem);
    number = Math.floor(number / base);
  }

  while(!stack.isEmpty()) {
    baseString += digits[stack.pop()];
  }

  return baseString;

}
