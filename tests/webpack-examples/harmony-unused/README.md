This example demonstrates how webpack tracks the usage of ES6 imports and exports. Only used exports are emitted to the resulting bundle. The minimizing step then removes the declarations because they are unused.

Excluding unused exports from bundles is known as "[tree-shaking](http://www.2ality.com/2015/12/webpack-tree-shaking.html)".

In this example, only `add` and `multiply` in `./math.js` are used by the app. `list` is unused and is not included in the minimized bundle (Look for `Array.from` in the minimized bundle).

In addition to that, `library.js` simulates an entry point to a big library. `library.js` re-exports multiple identifiers from submodules. Often big parts of that are unused, like `abc.js`. Note how the usage information flows from `example.js` through `library.js` into `abc.js` and all declarations in `abc.js` are not included in the minimized bundle (Look for `console.log("a")` in the minimized bundle).

# example.js

```javascript
import { add } from './math';
import * as library from "./library";

add(1, 2);
library.reexportedMultiply(1, 2);
```

# math.js

```javascript
export function add() {
	var sum = 0, i = 0, args = arguments, l = args.length;
	while (i < l) {
		sum += args[i++];
	}
	return sum;
}

export function multiply() {
	var product = 1, i = 0, args = arguments, l = args.length;
	while (i < l) {
		product *= args[i++];
	}
	return product;
}

export function list() {
	return Array.from(arguments);
}
```

# library.js

```javascript
export { a, b, c } from "./abc";
export { add as reexportedAdd, multiply as reexportedMultiply } from "./math";
```

# dist/output.js

```javascript
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ([
/* 0 */,
/* 1 */
/*!*****************!*\
  !*** ./math.js ***!
  \*****************/
/*! namespace exports */
/*! export add [provided] [no usage info] [missing usage info prevents renaming] */
/*! export list [provided] [no usage info] [missing usage info prevents renaming] */
/*! export multiply [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__.r, __webpack_exports__, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* ESM export */ __webpack_require__.d(__webpack_exports__, {
/* ESM export */   "add": () => (/* binding */ add),
/* ESM export */   "multiply": () => (/* binding */ multiply),
/* ESM export */   "list": () => (/* binding */ list)
/* ESM export */ });
function add() {
	var sum = 0, i = 0, args = arguments, l = args.length;
	while (i < l) {
		sum += args[i++];
	}
	return sum;
}

function multiply() {
	var product = 1, i = 0, args = arguments, l = args.length;
	while (i < l) {
		product *= args[i++];
	}
	return product;
}

function list() {
	return Array.from(arguments);
}


/***/ }),
/* 2 */
/*!********************!*\
  !*** ./library.js ***!
  \********************/
/*! namespace exports */
/*! export a [provided] [no usage info] [missing usage info prevents renaming] -> ./abc.js .a */
/*! export b [provided] [no usage info] [missing usage info prevents renaming] -> ./abc.js .b */
/*! export c [provided] [no usage info] [missing usage info prevents renaming] -> ./abc.js .c */
/*! export reexportedAdd [provided] [no usage info] [missing usage info prevents renaming] -> ./math.js .add */
/*! export reexportedMultiply [provided] [no usage info] [missing usage info prevents renaming] -> ./math.js .multiply */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_exports__, __webpack_require__.d, __webpack_require__.r, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* ESM export */ __webpack_require__.d(__webpack_exports__, {
/* ESM export */   "a": () => (/* reexport safe */ _abc__WEBPACK_IMPORTED_MODULE_0__.a),
/* ESM export */   "b": () => (/* reexport safe */ _abc__WEBPACK_IMPORTED_MODULE_0__.b),
/* ESM export */   "c": () => (/* reexport safe */ _abc__WEBPACK_IMPORTED_MODULE_0__.c),
/* ESM export */   "reexportedAdd": () => (/* reexport safe */ _math__WEBPACK_IMPORTED_MODULE_1__.add),
/* ESM export */   "reexportedMultiply": () => (/* reexport safe */ _math__WEBPACK_IMPORTED_MODULE_1__.multiply)
/* ESM export */ });
/* ESM import */ var _abc__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./abc */ 3);
/* ESM import */ var _math__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./math */ 1);



/***/ }),
/* 3 */
/*!****************!*\
  !*** ./abc.js ***!
  \****************/
/*! namespace exports */
/*! export a [provided] [no usage info] [missing usage info prevents renaming] */
/*! export b [provided] [no usage info] [missing usage info prevents renaming] */
/*! export c [provided] [no usage info] [missing usage info prevents renaming] */
/*! other exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__.r, __webpack_exports__, __webpack_require__.d, __webpack_require__.* */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* ESM export */ __webpack_require__.d(__webpack_exports__, {
/* ESM export */   "a": () => (/* binding */ a),
/* ESM export */   "b": () => (/* binding */ b),
/* ESM export */   "c": () => (/* binding */ c)
/* ESM export */ });
function a() { console.log("a"); }
function b() { console.log("b"); }
function c() { console.log("c"); }


/***/ })
/******/ 	]);
```

<details><summary><code>/* webpack runtime code */</code></summary>

``` js
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for ESM exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
```

</details>

``` js
var __webpack_exports__ = {};
// This entry need to be wrapped in an IIFE because it need to be isolated against other modules in the chunk.
(() => {
/*!********************!*\
  !*** ./example.js ***!
  \********************/
/*! namespace exports */
/*! exports [not provided] [no usage info] */
/*! runtime requirements: __webpack_require__, __webpack_require__.r, __webpack_exports__, __webpack_require__.* */
__webpack_require__.r(__webpack_exports__);
/* ESM import */ var _math__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./math */ 1);
/* ESM import */ var _library__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./library */ 2);



(0,_math__WEBPACK_IMPORTED_MODULE_0__.add)(1, 2);
_library__WEBPACK_IMPORTED_MODULE_1__.reexportedMultiply(1, 2);

})();

/******/ })()
;
```

# dist/output.js

```javascript
(()=>{"use strict";var r,e={451:(r,e,t)=>{function o(){for(var r=0,e=0,t=arguments,o=t.length;e<o;)r+=t[e++];return r}function n(){for(var r=1,e=0,t=arguments,o=t.length;e<o;)r*=t[e++];return r}t.d(e,{IH:()=>o,Jp:()=>n})}},t={};function o(r){var n=t[r];if(void 0!==n)return n.exports;var p=t[r]={exports:{}};return e[r](p,p.exports,o),p.exports}o.d=(r,e)=>{for(var t in e)o.o(e,t)&&!o.o(r,t)&&Object.defineProperty(r,t,{enumerable:!0,get:e[t]})},o.o=(r,e)=>Object.prototype.hasOwnProperty.call(r,e),(0,(r=o(451)).IH)(1,2),r.Jp(1,2)})();
```

# Info

## Unoptimized

```
asset output.js 7.03 KiB [emitted] (name: main)
chunk (runtime: main) output.js (main) 698 bytes (javascript) 670 bytes (runtime) [entry] [rendered]
  > ./example.js main
  dependent modules 584 bytes [dependent] 3 modules
  runtime modules 670 bytes 3 modules
  ./example.js 114 bytes [built] [code generated]
    [no exports]
    [used exports unknown]
    entry ./example.js main
webpack 5.51.1 compiled successfully
```

## Production mode

```
asset output.js 536 bytes [emitted] [minimized] (name: main)
chunk (runtime: main) output.js (main) 461 bytes (javascript) 396 bytes (runtime) [entry] [rendered]
  > ./example.js main
  runtime modules 396 bytes 2 modules
  dependent modules 347 bytes [dependent] 1 module
  ./example.js 114 bytes [built] [code generated]
    [no exports]
    [no exports used]
    entry ./example.js main
webpack 5.51.1 compiled successfully
```
