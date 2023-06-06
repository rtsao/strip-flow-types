import { expect, test } from "vitest";
import { transform } from "../index.js";

test("whitespace preservation", () => {
  expect(transform("function test(a: number) {}")).toMatchInlineSnapshot(
    '"function test(a        ) {}"'
  );

  expect(
    transform(`
  type abcd = number;

  function test<A,B>(foo: Dog<A>, bar: number) : abcd {
    const baz: number = 24;
    return foobar;
  };

  `)
  ).toMatchInlineSnapshot(`
    "
                         

      function test     (foo        , bar        )        {
        const baz         = 24;
        return foobar;
      };

      "
  `);

  expect(
    transform(`declare class URL {
    constructor(urlStr: string): URL;
    toString(): string;

    static compare(url1: URL, url2: URL): boolean;
  }`)
  ).toMatchInlineSnapshot(`
    "                   
                                         
                           

                                                      
       "
  `);

  expect(
    transform(`import type foo from "bar";`)
  ).toMatchInlineSnapshot('"                           "');

  expect(
    transform(`import type from "bar";`)
  ).toMatchInlineSnapshot('"import type from \\"bar\\";"');

  expect(
    transform(`import {type foo} from "bar";`)
  ).toMatchInlineSnapshot('"                             "');

  expect(
    transform(`import {bar, type foo} from "bar";`)
  ).toMatchInlineSnapshot('"import {bar          } from \\"bar\\";"');

  expect(
    transform(`import {bar, type foo, baz} from "bar";`)
  ).toMatchInlineSnapshot('"import {bar          , baz} from \\"bar\\";"');

  expect(
    transform(`import {bar as wat, type foo, baz} from "bar";`)
  ).toMatchInlineSnapshot('"import {bar as wat          , baz} from \\"bar\\";"');

  expect(
    transform(`import {type foo, baz} from "bar";`)
  ).toMatchInlineSnapshot('"import {          baz} from \\"bar\\";"');

  expect(
    transform(`import abc, {foo, type baz} from "bar";`)
  ).toMatchInlineSnapshot('"import abc, {foo          } from \\"bar\\";"');

  expect(
    transform(`export type {foo, baz} from "bar";`)
  ).toMatchInlineSnapshot('"                                  "');

  expect(
    transform(`export {type foo, baz} from "bar";`)
  ).toMatchInlineSnapshot('"export {          baz} from \\"bar\\";"');

  expect(
    transform(`export {foo, type baz} from "bar";`)
  ).toMatchInlineSnapshot('"export {foo          } from \\"bar\\";"');

  expect(
    transform(`export {type baz} from "bar";`)
  ).toMatchInlineSnapshot('"                             "');

  expect(
    transform(`function foo(a?) {}`)
  ).toMatchInlineSnapshot('"function foo(a ) {}"');

  expect(
    transform(`function foo(a?: number) {}`)
  ).toMatchInlineSnapshot('"function foo(a         ) {}"');

  expect(
    transform(`export type { foo, bar };`)
  ).toMatchInlineSnapshot('"                         "');

  expect(
    transform(`export type foo = {
      bar: ([foo, foo]) => [bar, bar]
    };`)
  ).toMatchInlineSnapshot(`
    "                   
                                         
          "
  `);
});

// tree-sitter-typescript does not correctly parse this syntax
test.fails("failing: ?(T) => T syntax", () => {
  expect(
    transform(`function normalizeProvider(provider: mixed, onError: ?(MapsSDKErrorEvent) => mixed): Provider {};`)
  ).toMatchInlineSnapshot('"function normalizeProvider(provider       , onError                               )           {};"');
});

// The error recovery mechanism of tree-sitter tolerates this TS syntax error, but a sufficiently long arrow functon will no longer parse as type_alias_declaration
test.fails("failing: export type assignment with sufficiently long arrow function", () => {
  expect(
    transform(`export type foo = {
      bar: ([number, number]) => [number, number]
    };`)
  ).toMatchInlineSnapshot(`
    "                   
                                                     
          "
  `);
});
