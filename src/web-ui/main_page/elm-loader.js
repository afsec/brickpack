console.log("elm-loader.js loaded")

var mainApp = Elm.Main.init({
    flags: {
        x: window.innerWidth,
        y: window.innerHeight,
        ua: navigator.userAgent,
        isEmptyPage: false
    },
    node: document.getElementById("app")
});

// app.ports.searchSomeData.subscribe(function (message) {
//     console.log("JS searchSomeData(): message=", message)
// });

mainApp.ports.consoleLog.subscribe(function (message) {
    console.log("JS consoleLog(): message=", message)
});

mainApp.ports.consoleError.subscribe(function (message) {
    console.error("JS consoleError(): message=", message)
});

mainApp.ports.newBrowserTab.subscribe(url => window.open(url, '_blank'));

////////////////////////////////////////////////////
// mainApp.ports.openCodeOnMonacoEditor.subscribe(function (code) {
//     console.log("JS openCodeOnMonacoEditor(): code=", code)
//     setTimeout(() => {
//         var monacoEditor = monaco.editor.create(document.getElementById('monaco-editor'), {
//             value: code,
//             language: 'lua',
//             theme: 'vs-dark'

//         });
//         // const codeEditorResult = code
//         const codeEditorResult = `
//     function main()
//         echo("<h1>It works!</h1>")
//     end

//     main()
//     `
//         console.log("JS openCodeOnMonacoEditor(): codeEditorResult=", codeEditorResult)
//         // Send data back to Elm
//         mainApp.ports.receiveCodeFromMonacoEditor.send(codeEditorResult)
//     }, 2000)
// });

