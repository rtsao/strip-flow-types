import { expect, test } from 'vitest'
import { sum } from '../index.js'

test('sum from native', () => {
  expect(sum(1, 2)).toBe(3)
})
