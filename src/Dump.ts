// @TODO: Rewrite this to typescript.

/* @ts-ignore */
Sfdump = window.Sfdump || (function
(doc) {
    let refStyle = doc.createElement("style"), rxEsc = /([.*+?^${}()|\[\]\/\\])/g,
        idRx                                         = /\bquo-dump-\d+-ref[012]\w+\b/,
        keyHint                                      = 0 <= navigator.platform.toUpperCase().indexOf("MAC") ? "Cmd" : "Ctrl",
        addEventListener                             = function (e, n, cb) {
            e.addEventListener(n, cb, false);
        };

    refStyle.innerHTML = "pre.quo-dump .quo-dump-compact, .quo-dump-str-collapse .quo-dump-str-collapse, .quo-dump-str-expand .quo-dump-str-expand { display: none; }";

    (doc.documentElement.firstElementChild || doc.documentElement.children[0]).appendChild(refStyle);

    refStyle = doc.createElement("style");

    (doc.documentElement.firstElementChild || doc.documentElement.children[0]).appendChild(refStyle);

    if (!doc.addEventListener) {
        addEventListener = function (element, eventName, callback) {
            element.attachEvent("on" + eventName, function (e) {
                e.preventDefault = function () {
                    e.returnValue = false;
                };
                e.target = e.srcElement;
                callback(e);
            });
        };
    }

    function toggle(a, recursive) {
        let s = a.nextSibling || {}, oldClass = s.className, arrow, newClass;
        if (/\bquo-dump-compact\b/.test(oldClass)) {
            arrow = "&#9660;";
            newClass = "quo-dump-expanded";
        } else if (/\bquo-dump-expanded\b/.test(oldClass)) {
            arrow = "&#9654;";
            newClass = "quo-dump-compact";
        } else {
            return false;
        }
        if (doc.createEvent && s.dispatchEvent) {
            const event = doc.createEvent("Event");
            event.initEvent("quo-dump-expanded" === newClass ? "sfbeforedumpexpand" : "sfbeforedumpcollapse", true, false);
            s.dispatchEvent(event);
        }
        a.lastChild.innerHTML = arrow;
        s.className = s.className.replace(/\bquo-dump-(compact|expanded)\b/, newClass);
        if (recursive) {
            try {
                a = s.querySelectorAll("." + oldClass);
                for (s = 0; s < a.length; ++s) {
                    if (-1 == a[s].className.indexOf(newClass)) {
                        a[s].className = newClass;
                        a[s].previousSibling.lastChild.innerHTML = arrow;
                    }
                }
            } catch (e) {
            }
        }
        return true;
    }

    function collapse(a, recursive) {
        const s = a.nextSibling || {}, oldClass = s.className;
        if (/\bquo-dump-expanded\b/.test(oldClass)) {
            toggle(a, recursive);
            return true;
        }
        return false;
    }

    function expand(a, recursive) {
        const s = a.nextSibling || {}, oldClass = s.className;
        if (/\bquo-dump-compact\b/.test(oldClass)) {
            toggle(a, recursive);
            return true;
        }
        return false;
    }

    function collapseAll(root) {
        const a = root.querySelector("a.quo-dump-toggle");
        if (a) {
            collapse(a, true);
            expand(a, true);
            return true;
        }
        return false;
    }

    function reveal(node) {
        let previous, parents = [];
        while ((node = node.parentNode || {}) && (previous = node.previousSibling) && "A" === previous.tagName) {
            parents.push(previous);
        }
        if (0 !== parents.length) {
            parents.forEach(function (parent) {
                expand(parent, true);
            });
            return true;
        }
        return false;
    }

    function highlight(root, activeNode, nodes) {
        resetHighlightedNodes(root);
        Array.from(nodes || []).forEach(function (node: HTMLElement) {
            if (!/\bquo-dump-highlight\b/.test(node.className)) {
                node.className = node.className + " quo-dump-highlight";
            }
        });
        if (!/\bquo-dump-highlight-active\b/.test(activeNode.className)) {
            activeNode.className = activeNode.className + " quo-dump-highlight-active";
        }
    }

    function resetHighlightedNodes(root) {
        Array.from(root.querySelectorAll(".quo-dump-str, .quo-dump-key, .quo-dump-public, .quo-dump-protected, .quo-dump-private")).forEach(function (strNode: HTMLElement) {
            strNode.className = strNode.className.replace(/\bquo-dump-highlight\b/, "");
            strNode.className = strNode.className.replace(/\bquo-dump-highlight-active\b/, "");
        });
    }

    return function (root, x) {
        root = doc.getElementById(root);
        const indentRx = new RegExp("^(" + (root.getAttribute("data-indent-pad") || " ").replace(rxEsc, "\\$1") + ")+", "m");
        const options = {"maxDepth": 1, "maxStringLength": 160, "fileLinkFormat": false};
        let elt = root.getElementsByTagName("A"), len = elt.length, i = 0, s, h, t = [];

        while (i < len) {
            t.push(elt[i++]);
        }

        /* @ts-ignore */
        for (i in x) {
            options[i] = x[i];
        }

        function a(e, f) {
            addEventListener(root, e, function (e, n) {
                if ("A" == e.target.tagName) {
                    f(e.target, e);
                } else if ("A" == e.target.parentNode.tagName) {
                    f(e.target.parentNode, e);
                } else {
                    n = /\bquo-dump-ellipsis\b/.test(e.target.className) ? e.target.parentNode : e.target;
                    if ((n = n.nextElementSibling) && "A" == n.tagName) {
                        if (!/\bquo-dump-toggle\b/.test(n.className)) {
                            n = n.nextElementSibling || n;
                        }
                        f(n, e, true);
                    }
                }
            });
        }

        function isCtrlKey(e) {
            return e.ctrlKey || e.metaKey;
        }

        function xpathString(str) {
            const parts = str.match(/[^'"]+|['"]/g).map(function (part) {
                if ("'" == part) {
                    return "\"'\"";
                }
                if ("\"" == part) {
                    return "'\"'";
                }
                return "'" + part + "'";
            });
            return "concat(" + parts.join(",") + ", '')";
        }

        function xpathHasClass(className) {
            return "contains(concat(' ', normalize-space(@class), ' '), ' " + className + " ')";
        }

        addEventListener(root, "mouseover", function (e) {
            if ("" != refStyle.innerHTML) {
                refStyle.innerHTML = "";
            }
        });
        a("mouseover", function (a, e, c) {
            if (c) {
                e.target.style.cursor = "pointer";
            } else if (a = idRx.exec(a.className)) {
                try {
                    refStyle.innerHTML = "pre.quo-dump ." + a[0] + "{background-color: #B729D9; color: #FFF !important; border-radius: 2px}";
                } catch (e) {
                }
            }
        });
        a("click", function (a, e, c) {
            if (/\bquo-dump-toggle\b/.test(a.className)) {
                e.preventDefault();
                if (!toggle(a, isCtrlKey(e))) {
                    let r = doc.getElementById(a.getAttribute("href").slice(1)), s = r.previousSibling,
                        f                                                          = r.parentNode, t = a.parentNode;
                    t.replaceChild(r, a);
                    f.replaceChild(a, s);
                    t.insertBefore(s, r);
                    /* @ts-ignore */
                    f = f.firstChild.nodeValue.match(indentRx);
                    t = t.firstChild.nodeValue.match(indentRx);
                    if (f && t && f[0] !== t[0]) {
                        r.innerHTML = r.innerHTML.replace(new RegExp("^" + f[0].replace(rxEsc, "\\$1"), "mg"), t[0]);
                    }
                    if (/\bquo-dump-compact\b/.test(r.className)) {
                        toggle(s, isCtrlKey(e));
                    }
                }
                if (c) {
                } else if (doc.getSelection) {
                    try {
                        doc.getSelection().removeAllRanges();
                    } catch (e) {
                        doc.getSelection().empty();
                    }
                } else {
                    /* @ts-ignore */
                    doc.selection.empty();
                }
            } else if (/\bquo-dump-str-toggle\b/.test(a.className)) {
                e.preventDefault();
                e = a.parentNode.parentNode;
                e.className = e.className.replace(/\bquo-dump-str-(expand|collapse)\b/, a.parentNode.className);
            }
        });
        elt = root.getElementsByTagName("SAMP");
        len = elt.length;
        i = 0;
        while (i < len) {
            t.push(elt[i++]);
        }
        len = t.length;
        for (i = 0; i < len; ++i) {
            elt = t[i];
            if ("SAMP" == elt.tagName) {
                /* @ts-ignore */
                a = elt.previousSibling || {};
                /* @ts-ignore */
                if ("A" != a.tagName) {
                    /* @ts-ignore */
                    a = doc.createElement("A");
                    a.className = "quo-dump-ref";
                    elt.parentNode.insertBefore(a, elt);
                } else {
                    /* @ts-ignore */
                    a.innerHTML += " ";
                }
                a.title = (a.title ? a.title + "\n[" : "[") + keyHint + "+click] Expand all children";
                /* @ts-ignore */
                a.innerHTML += elt.className == "quo-dump-compact" ? "<span>&#9654;</span>" : "<span>&#9660;</span>";
                a.className += " quo-dump-toggle";
                x = 1;
                if ("quo-dump" != elt.parentNode.className) {
                    x += elt.parentNode.getAttribute("data-depth") / 1;
                }
                /* @ts-ignore */
            } else if (/\bquo-dump-ref\b/.test(elt.className) && (a = elt.getAttribute("href"))) {
                /* @ts-ignore */
                a = a.slice(1);
                elt.className += " " + a;
                if (/[\[{]$/.test(elt.previousSibling.nodeValue)) {
                    /* @ts-ignore */
                    a = a != elt.nextSibling.id && doc.getElementById(a);
                    try {
                        /* @ts-ignore */
                        s = a.nextSibling;
                        elt.appendChild(a);
                        s.parentNode.insertBefore(a, s);
                        if (/^[@#]/.test(elt.innerHTML)) {
                            elt.innerHTML += " <span>&#9654;</span>";
                        } else {
                            elt.innerHTML = "<span>&#9654;</span>";
                            elt.className = "quo-dump-ref";
                        }
                        elt.className += " quo-dump-toggle";
                    } catch (e) {
                        if ("&" == elt.innerHTML.charAt(0)) {
                            elt.innerHTML = "&#8230;";
                            elt.className = "quo-dump-ref";
                        }
                    }
                }
            }
        }

        if (0 >= options.maxStringLength) {
            return;
        }

        try {
            elt = root.querySelectorAll(".quo-dump-str");
            len = elt.length;
            i = 0;
            t = [];
            while (i < len) {
                t.push(elt[i++]);
            }
            len = t.length;
            for (i = 0; i < len; ++i) {
                elt = t[i];
                s = elt.innerText || elt.textContent;
                x = s.length - options.maxStringLength;
                if (0 < x) {
                    h = elt.innerHTML;
                    elt[elt.innerText ? "innerText" : "textContent"] = s.substring(0, options.maxStringLength);
                    elt.className += " quo-dump-str-collapse";
                    elt.innerHTML = "<span class=quo-dump-str-collapse>" + h + "<a class=\"quo-dump-ref quo-dump-str-toggle\" title=\"Collapse\"> &#9664;</a></span>" + "<span class=quo-dump-str-expand>" + elt.innerHTML + "<a class=\"quo-dump-ref quo-dump-str-toggle\" title=\"" + x + " remaining characters\"> &#9654;</a></span>";
                }
            }
        } catch (e) {
        }
    };
})(document);
