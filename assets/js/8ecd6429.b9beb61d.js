(self.webpackChunkdoc_ops=self.webpackChunkdoc_ops||[]).push([[2635],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return u},kt:function(){return d}});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var c=r.createContext({}),p=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},u=function(e){var t=p(e.components);return r.createElement(c.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,c=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),m=p(n),d=i,f=m["".concat(c,".").concat(d)]||m[d]||s[d]||a;return n?r.createElement(f,o(o({ref:t},u),{},{components:n})):r.createElement(f,o({ref:t},u))}));function d(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=m;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:i,o[1]=l;for(var p=2;p<a;p++)o[p]=n[p];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},9578:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return c},metadata:function(){return p},toc:function(){return u},default:function(){return m}});var r=n(2122),i=n(9756),a=(n(7294),n(3905)),o=["components"],l={},c="\ud83d\uddd3\ufe0f Team Identity Meeting Notes - 2020-07-29",p={unversionedId:"meeting-notes/2020-07-29",id:"meeting-notes/2020-07-29",isDocsHomePage:!1,title:"\ud83d\uddd3\ufe0f Team Identity Meeting Notes - 2020-07-29",description:"\ud83d\udc65 Participants",source:"@site/docs/meeting-notes/2020-07-29.md",sourceDirName:"meeting-notes",slug:"/meeting-notes/2020-07-29",permalink:"/docs/meeting-notes/2020-07-29",editUrl:"https://github.com/iotaledger/identity.rs/edit/dev/documentation/docs/meeting-notes/2020-07-29.md",tags:[],version:"current",frontMatter:{}},u=[{value:"\ud83d\udc65 Participants",id:"-participants",children:[]},{value:"\ud83d\udcac Discussion topics",id:"-discussion-topics",children:[]}],s={toc:u};function m(e){var t=e.components,n=(0,i.Z)(e,o);return(0,a.kt)("wrapper",(0,r.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"\ufe0f-team-identity-meeting-notes---2020-07-29"},"\ud83d\uddd3\ufe0f Team Identity Meeting Notes - 2020-07-29"),(0,a.kt)("h2",{id:"-participants"},"\ud83d\udc65 Participants"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"@Thoralf-M"),(0,a.kt)("li",{parentName:"ul"},"@nothingismagick"),(0,a.kt)("li",{parentName:"ul"},"@vidalattias"),(0,a.kt)("li",{parentName:"ul"},"@JelleMillenaar")),(0,a.kt)("h2",{id:"-discussion-topics"},"\ud83d\udcac Discussion topics"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Cryptographic Accumulators: "),(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},"Read up for next week (@vidalattias)"),(0,a.kt)("li",{parentName:"ul"},"Possibly do a PoC with that"))),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"DID Comms: "),(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},"What is going to be the communications layer?",(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},"Abstracted from the library"))),(0,a.kt)("li",{parentName:"ul"},"Base HTTP, MQTT implementations?"),(0,a.kt)("li",{parentName:"ul"},"Possibly use IOTA Nodes as Peer-to-peer relay server?"))),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Should Service Endpoints be encrypted to prevent spam?"),(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},"They are ment to be public info"),(0,a.kt)("li",{parentName:"ul"},"Possibily allow both encrypted and unencrypted"))),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Identity.ts mentions not posting human identities, should we?"),(0,a.kt)("ul",{parentName:"li"},(0,a.kt)("li",{parentName:"ul"},"Yes, outdated stance, we do have to protect human privacy as much as possible.")))))}m.isMDXComponent=!0}}]);