window.quoTunnel.on((e, message) => {
    let variableName = message.detail.variableName;
    let id = message.detail.id;
    let noOfNodes = 0;
    let totalVariables = 1;

    if (id > 0) {
        noOfNodes = document.querySelectorAll(`div[id=quo-${id}]`).length;
        totalVariables = variableName.split(",");
        variableName = totalVariables[noOfNodes];
    } else {
        totalVariables = ["entry"];
    }

    let variableStyleAble = variableName.includes("$")
        ? "var-style"
        : variableName.includes("()") && !variableName.includes("::")
            ? "func-style"
            : variableName.includes("::") || variableName.includes("new ")
                ? "class-style"
                : variableName.includes("[") || variableName.includes("new ")
                    ? "array-style"
                    : "";

    let displayVarName = variableName.length !== 0 ? "" : "display:none;";

    document.getElementById("quo").insertAdjacentHTML("afterbegin", `
        <div class="quo-dump-container" id="quo-${message.detail.id}">
            <div class="time">
                <span>${message.detail.backtrace}</span>
                <span>${message.detail.time}</span>
            </div>
            <div class="quo-actual-dump">
                <h3 class="quo-title">
                    <div class="file">
                        <div>
                            <span class="received">Received (arg #${++noOfNodes} of ${totalVariables.length})</span>
                            <div class="passed" style="${displayVarName}">
                                <div>
                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M13.172 12l-4.95-4.95 1.414-1.414L16 12l-6.364 6.364-1.414-1.414z"/></svg>
                                     <span class="${variableStyleAble}">${variableName}</span>
                                </div> 
                            </div>
                       </div>  
                       <div style="margin-top:.75rem;">
                           <span class="received">Calltag</span>
                           <div class="passed" style="${displayVarName}">
                               <div>
                                   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M7.784 14l.42-4H4V8h4.415l.525-5h2.011l-.525 5h3.989l.525-5h2.011l-.525 5H20v2h-3.784l-.42 4H20v2h-4.415l-.525 5h-2.011l.525-5H9.585l-.525 5H7.049l.525-5H4v-2h3.784zm2.011 0h3.99l.42-4h-3.99l-.42 4z"/></svg>
                                   <span>${message.detail.callTag}</span>
                               </div>
                           </div>
                       </div>
                    </div>
                </h3>
                <div class="dumps">
                    ${message.data}
                </div>
            </div>
        </div>
    `);

    newEntryHandler(id);
});

window.addEventListener("DOMContentLoaded", () => {
    document.getElementById("clearLog").addEventListener("click", function () {
        for (let container of document.querySelectorAll("[class=quo-dump-container]")) {
            container.remove();
        }
    });

    document.getElementById("search").addEventListener("keyup", function (e) {
        let searchValue = e.target.value;
        let canSearch = Boolean(searchValue);
        let resultNodes = null;
        let allNodes = document.querySelectorAll(`i[data-searchable]`);

        if (Boolean(searchValue)) {
            searchValue = searchValue.replace("$", "");
            resultNodes = document.querySelectorAll(`i[data-searchable*=${searchValue}]`);
            document.getElementById("searchResult").innerText = `Found ${resultNodes.length} result${resultNodes.length > 1 || resultNodes.length === 0 ? "s" : ""}`;
        } else {
            document.getElementById("searchResult").innerText = ``;
        }

        allNodes.forEach(node => {
            let canStay = true;
            let searchable = node.dataset.searchable;
            let dumpContainer = node.parentElement.parentElement.parentElement.parentElement;

            if (canSearch) {
                resultNodes.forEach(node => {
                    if (searchable === node.dataset.searchable) {
                        canStay = false;
                    }
                });
            } else {
                dumpContainer.style = "";
                return true;
            }


            if (canStay) {
                dumpContainer.style = "display:none!important;";
            } else {
                dumpContainer.style = "";
            }
        });
    });
});

function newEntryHandler(id) {
    let dump = document.querySelector(`div[id*=quo-${id}] pre[id*=quo-dump-]`);
    Sfdump(dump.id);
    let prev = null;
    Array.from(document.getElementsByTagName("article")).reverse().forEach(function (article) {
        const dedupId = article.dataset.dedupId;
        if (dedupId === prev) {
            article.getElementsByTagName("header")[0].classList.add("hidden");
        }
        prev = dedupId;
    });
}
