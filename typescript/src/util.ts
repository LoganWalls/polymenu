/**
* Contains utility functions for building UI components. 
*
* @module util
* */

/** 
 * Useful for moving a cursor around the options in a list.
 *
 * @param i - The index to wrap
 * @param shift - How much to shift the index by
 * @param low - The lowest value that can be returned
 * @param high - The highest value that can be returned
 *
 * @returns `i` shifted by `shift`, wrapping around `low` and `high`.
 */
export function wrappingShift(i: number, shift: number, low: number, high: number): number {
  i += shift;
  if (i > high) {
    i = low + (i - high) - 1;
  }
  if (i < low) {
    i = high - (low - i) + 1;
  }
  return i
}

/**
 * Adds an `item` to the `set` if it's not already there, otherwise removes it.
 *
 * @param item - The item to add or remove from the set.
 * @param set - The set to modify.
 * @param getKey - A function that returns the key used to compare `item` to the elements in `set`.
 *
 * @returns `set` (after adding / removing `item`)
 */
export function toggleSet<T>(item: T, set: T[], getKey: (i: T) => any = (item) => item): T[] {
  if (set.includes(item)) {
    set = set.filter((item) => getKey(item) !== getKey(item));
  } else {
    set.push(item);
  }
  return set
}

