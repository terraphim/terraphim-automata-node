import test from 'ava'

import { findMatched, replaceAllStream } from '../index'

test('test matcher native code', (t) => {
  const fixture_matched = ['let', 'it', 'be', 'be']
  const matched = findMatched(['let', 'it', 'be'], 'let it be be not found')

  t.is(matched[3], fixture_matched[3])
})
test('test replacer native code', (t) => {
  const fixture_replaced = '-let -it -be -be not found'
  t.is(replaceAllStream(['let', 'it', 'be'], ['-let', '-it', '-be'], 'let it be be not found'), fixture_replaced)
})
