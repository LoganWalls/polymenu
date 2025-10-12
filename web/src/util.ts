export function wrappingShift(v: number, shift: number, low: number, high: number): number {
  v += shift;
  if (v > high) {
    v = low + (v - high) - 1;
  }
  if (v < low) {
    v = high - (low - v) + 1;
  }
  return v
}

export function toggleSet<T>(item: T, set: T[], getKey: (i: T) => any = (item) => item): T[] {
  if (set.includes(item)) {
    set = set.filter((item) => getKey(item) !== getKey(item));
  } else {
    set.push(item);
  }
  return set
}

export default {
  wrappingShift,
  toggleSet,
}
