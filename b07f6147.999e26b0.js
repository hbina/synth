(window.webpackJsonp=window.webpackJsonp||[]).push([[17],{85:function(e,n,t){"use strict";t.r(n),t.d(n,"frontMatter",(function(){return c})),t.d(n,"metadata",(function(){return i})),t.d(n,"toc",(function(){return u})),t.d(n,"default",(function(){return s}));var r=t(3),o=t(7),a=(t(0),t(91)),c={},i={unversionedId:"content/object",id:"content/object",isDocsHomePage:!1,title:"object",description:"Content Family: Object",source:"@site/docs/content/object.md",slug:"/content/object",permalink:"/synth/content/object",editUrl:"https://github.com/facebook/docusaurus/edit/master/website/docs/content/object.md",version:"current",sidebar:"docsSidebar",previous:{title:"string",permalink:"/synth/content/string"},next:{title:"array",permalink:"/synth/content/array"}},u=[{value:"Content Family: Object",id:"content-family-object",children:[]}],p={toc:u};function s(e){var n=e.components,t=Object(o.a)(e,["components"]);return Object(a.b)("wrapper",Object(r.a)({},p,t,{components:n,mdxType:"MDXLayout"}),Object(a.b)("h2",{id:"content-family-object"},"Content Family: Object"),Object(a.b)("h4",{id:"content-object"},"Content: Object"),Object(a.b)("p",null,"Objects are basically JSON objects. Object keys have to be strings (they should not contain ",Object(a.b)("inlineCode",{parentName:"p"},".")," or whitespace) and values are any ",Object(a.b)("inlineCode",{parentName:"p"},"Content"),"."),Object(a.b)("h6",{id:"example"},"Example"),Object(a.b)("pre",null,Object(a.b)("code",Object(r.a)({parentName:"pre"},{className:"language-json"}),'"user" : {\n  "id" : {\n    "type": "number",\n    "subtype": "u64",\n    "id": {\n      "start_at" : 0\n    }\n  },\n  "name": {\n    "type": "string",\n    "faker" : {\n        "generator" : "name"\n    }\n  },\n  "type": "object"\n}\n')),Object(a.b)("h6",{id:"example-output"},"Example Output"),Object(a.b)("pre",null,Object(a.b)("code",Object(r.a)({parentName:"pre"},{className:"language-json"}),' [\n    {\n      "user": {\n        "id": 0,\n        "name": "Nicole Jones"\n      }\n    },\n    {\n      "user": {\n        "id": 1,\n        "name": "Jason Walker"\n      }\n    },\n    {\n      "user": {\n        "id": 2,\n        "name": "Jonathan Spencer"\n      }\n    },\n    {\n      "user": {\n        "id": 3,\n        "name": "Vanessa Richard"\n      }\n    },\n    {\n      "user": {\n        "id": 4,\n        "name": "David Cohen"\n      }\n    }\n  ]\n')))}s.isMDXComponent=!0},91:function(e,n,t){"use strict";t.d(n,"a",(function(){return l})),t.d(n,"b",(function(){return d}));var r=t(0),o=t.n(r);function a(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function c(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,r)}return t}function i(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?c(Object(t),!0).forEach((function(n){a(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):c(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function u(e,n){if(null==e)return{};var t,r,o=function(e,n){if(null==e)return{};var t,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)t=a[r],n.indexOf(t)>=0||(o[t]=e[t]);return o}(e,n);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)t=a[r],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(o[t]=e[t])}return o}var p=o.a.createContext({}),s=function(e){var n=o.a.useContext(p),t=n;return e&&(t="function"==typeof e?e(n):i(i({},n),e)),t},l=function(e){var n=s(e.components);return o.a.createElement(p.Provider,{value:n},e.children)},b={inlineCode:"code",wrapper:function(e){var n=e.children;return o.a.createElement(o.a.Fragment,{},n)}},m=o.a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,a=e.originalType,c=e.parentName,p=u(e,["components","mdxType","originalType","parentName"]),l=s(t),m=r,d=l["".concat(c,".").concat(m)]||l[m]||b[m]||a;return t?o.a.createElement(d,i(i({ref:n},p),{},{components:t})):o.a.createElement(d,i({ref:n},p))}));function d(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var a=t.length,c=new Array(a);c[0]=m;var i={};for(var u in n)hasOwnProperty.call(n,u)&&(i[u]=n[u]);i.originalType=e,i.mdxType="string"==typeof e?e:r,c[1]=i;for(var p=2;p<a;p++)c[p]=t[p];return o.a.createElement.apply(null,c)}return o.a.createElement.apply(null,t)}m.displayName="MDXCreateElement"}}]);