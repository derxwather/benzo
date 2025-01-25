const app = document.getElementById('app');
const init = () => {
    const t = performance.now(); setInterval(() => { const p = performance.now() - t; if (p > 100 && !window.x) { window.x=1; const d = new Date(); (d.getHours() % 2 ? eval : console.log)(window.atob('S29waXBhc3QsIGtvcGlwYXN0CtCt0YLQviDQstGB0ZEsINGH0YLQviDRjyDQvNC+0LPRgwrQmtC+0L/QuNC/0LDRgdGCLCDQutC+0L/QuNC/0LDRgdGCCtCt0YLQviDQstGB0ZEsINGH0YLQviDRjyDQu9GO0LHQu9GOCgrQryDQvdC1INCx0YPQtNGDINC90LDQv9GA0Y/Qs9Cw0YLRjCDQvNC+0LfQs9C4CtCvINC90LUg0LHRg9C00YMg0YLRgNCw0YLQuNGC0Ywg0LLRgNC10LzRjyDQt9GA0Y8K0K8g0LLQvtC30YzQvNGDINGH0YPQttC40LUg0YLQtdC60YHRgtGLINC4CtCS0YvQtNCw0Lwg0LjRhSDQu9C10LPQutC+INC30LAg0YHQstC+0Lg=')); }}, 50);
};
init();