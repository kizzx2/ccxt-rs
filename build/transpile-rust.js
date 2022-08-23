"use strict";

const acorn = require('acorn');
const walk = require('acorn-walk');
const {
    unCamelCase,
    precisionConstants,
    safeString,
} = require('../js/base/functions.js');
const { truncate } = require('../js/base/functions/number.js');

var THE_GLOBAL_METHOD;

function isUpperCase(x) {
    return x && x.length > 0 && x[0] === x.toUpperCase()[0];
}

function isAllCaps(x) {
    if (!x) {
        return false;
    }
    for (let i = 0; i < x.length; i++) {
        if (x[i] !== x[i].toUpperCase()) {
            return false;
        }
    }
    return true;
}

function unCamelCamelCase(x) {
    return !x || x.length === 0 || isUpperCase(x) ? x : unCamelCase(x);
}

function getFunctionNameFromCallee(node) {
    switch (node.type) {
        case "MemberExpression":
            if (node.property.type !== 'Identifier') {
                throw new Error("Unexpected MemberExpression");
            }
            return node.property.name;
        case "Identifier":
            return node.name;
        default:
            throw new Error("Unexpected callee type");
    }
}

function transformIdentifier(name) {
    switch (name) {
        case 'type':
            return "r#type";

        default:
            return unCamelCamelCase(name);
    }
}

function quoteString(str) {
    return str.includes('"') ? `r#"${str}"#` : `"${str}"`;
}

function getCalleeFunctionName(node) {
    if (node.type !== 'CallExpression') {
        throw new Error("Unexpected node type");
    }
    switch (node.callee.type) {
        case "MemberExpression":
            if (node.callee.property.type !== 'Identifier') {
                throw new Error("Unexpected MemberExpression");
            }
            return node.callee.property.name;
        case "Identifier":
            return node.callee.name;
        default:
            throw new Error("Unexpected callee type");
    }
}

function getReturnType(node) {
    if (node.type !== 'CallExpression') {
        throw new Error("Unexpected node type");
    }

    const fname = getCalleeFunctionName(node);
    switch (fname) {
        case 'stringEquals':
        case 'stringEq':
        case 'stringGt':
        case 'stringGe':
        case 'stringLt':
        case 'stringLe':
            return 'bool';

        default:
            return 'value';
    }
}

function getArgumentCount(node) {
    if (node.type !== 'CallExpression') {
        throw new Error("Unexpected node type");
    }
    const argCounts = {
        safeValue: 3,
        fetchAccounts: 1,
        fetchTrades: 4,
        sign: 6,
        parseMarketLeverageTiers: 2,
        parseTransaction: 2,
        parseTransfer: 2,
        safeString: 3,
        safeStringUpper: 3,
        safeStringLower: 3,
        parseNumber: 2,
        sortBy2: 4,
        safeString2: 4,
        safeStringLower2: 4,
        safeStringUpper2: 4,
        safeMarket: 3,
        safeNumber: 3,
        stringDiv: 3,
        safeInteger: 3,
        safeCurrencyCode: 2,
        convertTradingViewToOHLCV: 8,
        filterByArray: 4,
        cancelOrder: 3,
        filterBySymbolSinceLimit: 5,
        sortBy: 3,
        safeCurrency: 2,

        decimalToPrecision: 5,
        numberToString: 1,
        fetchTrades: 4,
        parseTicker: 2,
        filterByValueSinceLimit: 7,
        parseDepositAddress: 2,
        parseBorrowInterest: 2,
        parseFundingRateHistory: 2,
        totp: 1,
        parseTradingLimits: 3,
        parseTrade: 2,
        parseLedgerEntry: 2,
        parsePosition: 2,
        implodeParams: 2,
        extractParams: 2,
        fetchTradingLimitsById: 2,
        filterBySinceLimit: 5,
        aggregate: 1,
        parseOrder: 2,
        iso8601: 1,
        fetchBorrowRate: 2,
        loadMarkets: 2,
        fetchTime: 1,
        safeStringN: 3,
        fetchFundingRates: 2,
        fetchLeverageTiers: 2,
        buildOHLCVC: 4,
        fetch: 4,
        throttle: 1,
        safeTimestamp: 3,
        fetchDepositAddresses: 2,
        fetchBorrowRates: 1,
        fetchOrderBook: 3,
        fetchTradingLimits: 2,
    };
    return argCounts[getCalleeFunctionName(node)] ?? node.arguments.length;
}

module.exports = {
    transpileMethodToRust(className, method) {
        method = method.trim().startsWith('async ') ? method.replace(/^\s*async\s+/, 'async function ') : `function ${method}`;
        THE_GLOBAL_METHOD = method;
        const ast = acorn.parse(method, { ecmaVersion: 2017 });

        const output = { value: "" };
        let currentOutput = output;

        const emit = (x) => {
            currentOutput.value += x;
        };

        // const emitAs = (type, node, state, c) => {
        //     switch (type) {
        //         case 'value':
        //             switch (node.type) {
        //                 case 'BinaryExpression':
        //                     emit("(");
        //                     c(node, asType(state));
        //                     emit(").into()");
        //                     break;

        //                 case 'Literal':
        //                     c(node, asType(state));
        //                     emit(".into()"); // XXX Probably isn't needed?
        //                     break;

        //                 default:
        //                     c(node, asType(state));
        //                     if (isAllCaps(node.name)) {
        //                         emit(".into()");
        //                     } else {
        //                         emit(".clone()");
        //                     }
        //                     break;
        //             }
        //             break;

        //         case 'usize':
        //             switch (node.type) {
        //                 case 'Literal':
        //                     c(node, asType(state));
        //                     break;

        //                 default:
        //                     c(node, asType(state));
        //                     emit(".unwrap_json().as_u64().unwrap().into()");
        //                     break;
        //             }
        //             break;

        //         case 'bool':
        //             switch (node.type) {
        //                 case 'Literal':
        //                     c(node, asType(state));
        //                     break;

        //                 case 'Identifier':
        //                 case 'CallExpression':
        //                     c(node, asType(state));
        //                     emit(".into()");
        //                     break;

        //                 default:
        //                     c(node, asType(state));
        //                     break;
        //             }
        //             break;

        //         default:
        //             throw new Error("Unexpected emitAs type");
        //     }
        // };

        const asType = (state, type) => ({
            ...state,
            asType: type
        });

        const withNewOutput = (node, state, c) => {
            const oldOutput = currentOutput;
            const rv = { value: "" };
            currentOutput = rv;
            c(node, asType(state));
            currentOutput = oldOutput;
            return rv.value;
        };

        const indent = (state) => {
            emit(' '.repeat(state.indentLevel * state.indentSize));
        }

        walk.recursive(ast, {
            indentLevel: 1,
            indentSize: 4,
            asType: undefined
        }, {
            FunctionDeclaration(node, state, c) {
                const params = [];
                const defaultValues = {};
                for (const param of node.params) {
                    switch (param.type) {
                        case 'Identifier':
                            switch (param.name) {
                                case 'type':
                                    params.push('r#type');
                                    break;
                                default:
                                    params.push(unCamelCamelCase(param.name));
                                    break;
                            }
                            break;
                        case 'AssignmentPattern':
                            const n = unCamelCamelCase(param.left.name);
                            params.push(n);
                            defaultValues[n] = param.right.value;
                            break;
                        default:
                            throw new Error('Unsupported parameter type: ' + param.type);
                    }
                }

                indent(state);
                let retType = 'Value';
                const fname = node.id.name;
                const isAsync = fname.startsWith('fetch') ||
                    fname.startsWith('load') || // Walk the AST to find out if the function is async
                    fname.startsWith('edit') ||
                    fname.startsWith('create') ||
                    fname.startsWith('cancel') ||
                    fname === 'request';
                let isSelfImmutable = fname.startsWith('safe');
                if (fname === 'safeOrder' || fname === 'safeTrade') {
                    isSelfImmutable = false;
                }
                if (fname === 'commonCurrencyCode' || fname === 'parseAccount' || fname === 'market') {
                    isSelfImmutable = true;
                }

                if (fname.startsWith('throw')) {
                    retType = '()';
                }
                emit(`${isAsync ? 'async ' : ''}fn ${unCamelCamelCase(fname)} (&${isSelfImmutable ? '' : 'mut '}self, ${params.map((x) => `mut ${x}: Value`).join(', ')}) -> ${retType} `);

                if (node.body.body.length === 0) {
                    emit("{ Value::Undefined }");
                } else {
                    c(node.body, asType({
                        ...state,
                        functionName: fname,
                        indentLevel: state.indentLevel + 1
                    }));
                }
            },

            BlockStatement(node, state, c) {
                emit("{\n");
                const appendBlock = state.appendBlock;
                state.appendBlock = undefined;

                for (const stmt of node.body) {
                    indent(state);
                    c(stmt, asType(state));
                    emit(";\n");
                }

                if (appendBlock) {
                    indent(state);
                    emit(appendBlock);
                    emit("\n");
                }

                indent({
                    ...state,
                    indentLevel: state.indentLevel - 1
                });
                emit("}");
            },

            ThrowStatement(node, state, c) {
                emit('panic!(r###"');
                c(node.argument, asType(state));
                emit('"###)');
            },

            NewExpression(node, state, c) {
                c(node.callee, asType(state));
                emit("::new(");
                for (let i = 0; i < node.arguments.length; i++) {
                    const arg = node.arguments[i];
                    c(arg, asType(state));
                    if (i < node.arguments.length - 1) {
                        emit(", ");
                    }
                }
                emit(")");
            },

            ExpressionStatement(node, state, c) {
                c(node.expression, asType(state));
            },

            AwaitExpression(node, state, c) {
                // XXX Another special case
                const callee = node.argument.callee;
                let shouldAwait = true;
                if (
                    callee.object.type === 'ThisExpression' &&
                    callee.property.type === 'Identifier' &&
                    callee.property.name === 'market'
                ) {
                    shouldAwait = false;
                }

                c(node.argument, asType({
                    ...state,
                    awaited: shouldAwait
                }, state.asType));
            },

            AssignmentExpression(node, state, c) {
                // Special case
                if (
                    node.left.type === 'MemberExpression' &&
                    node.left.object.type === 'ThisExpression' &&
                    node.left.property.type === 'Identifier' &&
                    node.left.property.name === 'number' &&
                    node.right.type === 'Identifier' && node.right.name === 'String'
                ) {
                    emit(`self.set_number_mode("String".into())`);
                    return;
                }

                if (node.left.type === 'MemberExpression') {
                    if (node.left.property.type !== 'Identifier' && node.left.property.type !== 'Literal' && node.left.property.type !== 'MemberExpression') {
                        throw new Error("Unexpected MemberExpression");
                    }

                    c(node.left.object, asType(state));
                    emit(".set(")
                    switch (node.left.property.type) {
                        case 'Literal':
                            c(node.left.property, asType(state));
                            emit('.into()');
                            break;

                        case 'Identifier':
                            // emit('"');
                            if (node.left.computed) {
                                c(node.left.property, asType(state));
                                emit('.clone()');
                            } else {
                                emit('"');
                                c(node.left.property, asType(state));
                                emit('".into()');
                            }
                            break;

                        default:
                            c(node.left.property, asType(state));
                            break;
                    }
                    emit(", ");
                    c(node.right, asType(state, 'rvalue'));
                    emit(")");
                    return;
                }

                c(node.left, asType(state));
                emit(' ');

                switch (node.operator) {
                    case '=':
                        emit(node.operator);
                        break;

                    case '+=':
                        emit('= ');
                        c(node.left, asType(state));
                        emit(' + ');
                        break;

                    case '-=':
                        emit('= ');
                        c(node.left, asType(state));
                        emit(' - ');
                        break;

                    case '*=':
                        emit('= ');
                        c(node.left, asType(state));
                        emit(' * ');
                        break;

                    case '/=':
                        emit('= ');
                        c(node.left, asType(state));
                        emit(' / ');
                        break;

                    default:
                        throw new Error("Unexpected assignment operator");
                }
                emit(' ');

                c(node.right, asType(state, 'value'));

                // switch (node.right.type) {
                //     case 'LogicalExpression':
                //         emit('(');
                //         c(node.right, state);
                //         emit(').into()');
                //         break;

                //     case 'Literal':
                //         c(node.right, state);
                //         emit(".into()");
                //         break;

                //     case 'Identifier':
                //         c(node.right, state);
                //         emit(".clone()");
                //         break;

                //     default:
                //         c(node.right, state);
                //         break;
                // }
            },

            VariableDeclaration(node, state, c) {
                for (const decl of node.declarations) {
                    c(decl, asType(state));
                }
            },

            VariableDeclarator(node, state, c) {
                let ident;
                switch (node.id.name) {
                    case 'type':
                        ident = 'r#type';
                        break;
                    default:
                        ident = unCamelCamelCase(node.id.name);
                        break;
                };
                if (state.parent?.type === 'ForStatement') {
                    emit(`let mut ${ident}: usize = `);
                    c(node.init, asType(state, 'usize'));
                } else {
                    emit(`let mut ${ident}: Value = `);
                    c(node.init, asType(state, 'value'));
                }
            },

            Literal(node, state, c) {
                switch (typeof node.value) {
                    case 'number':
                        emit(node.value.toString());
                        switch (state.asType) {
                            case undefined:
                            case 'usize':
                                break;

                            case 'property':
                            case 'value':
                                emit(".into()");
                                break;

                            default:
                                throw new Error("Unexpected literal type");
                        }
                        break;

                    case 'string':
                        switch (state.asType) {
                            case undefined:
                                break;

                            case 'value':
                            case 'property':
                                emit("Value::from(");
                                break;

                            default:
                                throw new Error("Unexpected literal type");
                        }
                        emit(quoteString(node.value));
                        switch (state.asType) {
                            case undefined:
                                break;

                            case 'value':
                            case 'property':
                                emit(")");
                                break;

                            default:
                                throw new Error("Unexpected literal type");
                        }
                        break;

                    case 'boolean':
                        emit(node.value ? 'true' : 'false');
                        switch (state.asType) {
                            case undefined:
                            case 'bool':
                                break;

                            case 'value':
                                emit(".into()");
                                break;

                            default:
                                throw new Error("Unexpected literal type");
                        }
                        break;

                    default:
                        throw new Error("Unexpected literal type");
                }
            },

            ConditionalExpression(node, state, c) {
                emit("if ");
                c(node.test, asType(state));
                if (node.test.type !== 'BinaryExpression') {
                    emit(".is_truthy()");
                }
                emit(" { ");
                c(node.consequent, asType(state, 'value'));
                emit(" } else { ");
                c(node.alternate, asType(state, 'value'));
                emit(" }");
            },

            UnaryExpression(node, state, c) {
                switch (node.operator) {
                    case '!':
                        switch (state.asType) {
                            case 'value':
                                emit("(");
                                break;

                            case 'bool':
                            case undefined:
                                break;

                            default:
                                throw new Error("Unexpected asType");
                        }

                        emit(node.operator);
                        c(node.argument, asType(state, 'bool'));

                        switch (state.asType) {
                            case 'value':
                                emit(").into()");
                                break;

                            case 'bool':
                            case undefined:
                                break;

                            default:
                                throw new Error("Unexpected asType");
                        }
                        break;

                    case 'typeof':
                        c(node.argument, asType(state));
                        emit(".typeof_()");
                        break;

                    default:
                        throw new Error("Unexpected unary operator");
                }
            },

            IfStatement(node, state, c) {
                // if (node.test.type === 'Identifier' || node.test.type === 'MemberExpression' || node.test.type === 'UnaryExpression') {
                //     emit('if (');
                //     c(node.test, state);
                //     if (node.test.type === 'UnaryExpression' && node.test.operator === '!') {
                //         emit(").is_falsy() ");
                //     } else {
                //         emit(").is_truthy() ");
                //     }
                // } else {
                //     emit('if ');
                //     c(node.test, state);
                //     emit(" ");
                // }

                emit("if ");
                c(node.test, asType(state, 'bool'));
                emit(" ");

                c(node.consequent, asType({
                    ...state,
                    indentLevel: state.indentLevel + 1
                }));
                if (node.alternate) {
                    emit(' else ');
                    if (node.alternate.type === 'IfStatement') {
                        c(node.alternate, asType({
                            ...state,
                            indentLevel: state.indentLevel
                        }));
                    } else {
                        c(node.alternate, asType({
                            ...state,
                            indentLevel: state.indentLevel + 1
                        }));
                    }
                }
            },

            LogicalExpression(node, state, c) {
                switch (node.operator) {
                    case '&&':
                    case '||':
                        switch (state.asType) {
                            case 'value':
                                emit("(");
                                break;

                            case undefined:
                            case 'bool':
                                break;

                            default:
                                throw new Error("Unexpected logical expression type");
                        }

                        c(node.left, asType(state, 'bool'));
                        emit(" ");
                        emit(node.operator);
                        emit(" ");
                        c(node.right, asType(state, 'bool'));

                        switch (state.asType) {
                            case 'value':
                                emit(").into()");
                                break;

                            case undefined:
                            case 'bool':
                                break;

                            default:
                                throw new Error("Unexpected logical expression type");
                        }
                        break;

                    default:
                        throw new Error("Unexpected logical operator");
                }
            },

            BinaryExpression(node, state, c) {
                // const leftIsUndefined = node.left.type === 'Identifier' && node.left.name === 'undefined';
                // const rightIsUndefined = node.right.type === 'Identifier' && node.right.name === 'undefined';

                // if (leftIsUndefined) {
                //     c(node.right, state);
                //     emit(".is_undefined()");
                //     return;
                // }

                // if (rightIsUndefined) {
                //     c(node.left, state);
                //     emit(".is_undefined()");
                //     return;
                // }

                switch (node.operator) {
                    case 'in':
                        c(node.right, asType(state));
                        emit(".contains_key(");
                        c(node.left, asType(state, 'value'));
                        emit(")");
                        switch (state.asType) {
                            case undefined:
                            case 'bool':
                                break;

                            case 'value':
                                emit(".into()");
                                break;

                            default:
                                throw new Error("Unexpected asType");
                        }
                        break;

                    case '===':
                    case '!==':
                    case '==':
                    case '!=':
                    case '>':
                    case '<':
                    case '>=':
                    case '<=':
                        if (state.asType === 'value') {
                            emit("(");
                        }
                        c(node.left, asType(state, 'value'));
                        emit(" ");
                        if (node.operator === '===') {
                            emit("==");
                        } else if (node.operator === '!==') {
                            emit("!=");
                        } else {
                            emit(node.operator);
                        }
                        emit(" ");
                        c(node.right, asType(state, 'value'));
                        if (state.asType === 'value') {
                            emit(").into()");
                        }
                        break;

                    case "+":
                    case "-":
                    case "*":
                    case "/":
                        c(node.left, asType(state, 'value'));
                        emit(" ");
                        emit(node.operator);
                        emit(" ");
                        c(node.right, asType(state, 'value'));
                        break;

                    default:
                        throw new Error("Unexpected binary operator: " + node.operator);
                }
            },

            CallExpression(node, state, c) {
                const argCounts = getArgumentCount(node);
                const retType = getReturnType(node);
                let shouldAwait = state.awaited;

                state = {
                    ...state,
                    awaited: undefined
                };

                c(node.callee, asType({
                    ...state,
                    parent: node
                }));
                emit("(");
                for (let i = 0; i < argCounts; i++) {
                    const arg = node.arguments[i];
                    if (!arg) {
                        emit("Value::Undefined");
                    } else {
                        c(arg, asType(state, 'value'));
                    }
                    if (i < argCounts - 1) {
                        emit(", ");
                    }
                }
                emit(")");
                switch (state.asType) {
                    case undefined:
                    case 'rvalue':
                    case 'value':
                        break;

                    case 'bool':
                        if (retType !== 'bool') {
                            emit(".is_truthy()");
                        }
                        break;

                    default:
                        throw new Error("Unexpected asType");
                }

                const fname = getFunctionNameFromCallee(node.callee);

                if (fname === 'cancelOrder' || fname === 'fetchTransactionFees' || fname === 'fetchTransactionFee') {
                    shouldAwait = true;
                }

                if (shouldAwait) {
                    emit(".await");
                }
            },

            MemberExpression(node, state, c) {
                if ((node.property.type === 'Literal') || (node.object.type === 'ThisExpression' && state.parent?.type !== 'CallExpression')) {
                    c(node.object, asType(state));
                    emit(`.get(`)
                    c(node.property, asType(state, 'property'));
                    emit(`)`);

                    switch (state.asType) {
                        case undefined:
                        case 'rvalue':
                        case 'value':
                            break;

                        case 'bool':
                            emit(".is_truthy()");
                            break;

                        default:
                            throw new Error("Unexpected asType");
                    }
                } else if (node.property.type === 'Identifier' && node.property.name === 'length') {
                    c(node.object, asType(state));
                    emit(".");
                    c(node.property, asType(state, 'member'));
                    switch (state.asType) {
                        case undefined:
                            break;

                        case 'value':
                            emit(".into()");
                            break;

                        default:
                            throw new Error("Unexpected asType");
                    }
                } else if (node.property.type === 'Identifier' && (node.property.name === 'i' || node.property.name === 'j')) {
                    c(node.object, asType(state));
                    emit(".get(");
                    c(node.property, asType(state));
                    emit(".into())");
                } else if (state.parent?.type === 'CallExpression' && node.property.type === 'Identifier' && node.property.name === 'extend') {
                    if (state.parent.arguments.length === 2) {
                        c(node.object, asType(state));
                        emit(".extend_2");
                    } else if (state.parent.arguments.length === 4) {
                        throw new Error('Unsupported extend call');
                        c(node.object, asType(state));
                        emit(".extend_4");
                    } else {
                        throw new Error('Unsupported extend call');
                    }
                } else if (state.parent?.type === 'CallExpression' && node.property.type === 'Identifier' && node.property.name === 'deepExtend') {
                    if (state.parent.arguments.length === 2) {
                        c(node.object, asType(state));
                        emit(".deep_extend_2");
                    } else if (state.parent.arguments.length === 4) {
                        c(node.object, asType(state));
                        emit(".deep_extend_4");
                    } else {
                        throw new Error('Unsupported deepExtend call');
                    }
                } else {
                    if (node.object.type === 'Identifier' && isUpperCase(node.object.name)) {
                        c(node.object, asType(state));
                        emit("::");
                        c(node.property, asType(state));
                    } else {
                        if (state.parent?.type !== 'CallExpression') {
                            c(node.object, asType(state));
                            emit(".get(");
                            c(node.property, asType(state));
                            emit(".clone())");

                            switch (state.asType) {
                                case 'bool':
                                    emit(".is_truthy()");
                                    break;

                                case 'value':
                                case undefined:
                                    break;

                                default:
                                    throw new Error("Unexpected asType");
                            }
                        } else {
                            c(node.object, asType({
                                ...state,
                                parent: node
                            }));
                            emit(".");
                            c(node.property, asType({
                                ...state,
                                parent: node
                            }));
                        }
                    }
                }
            },

            ThisExpression(node, state, c) {
                emit('self');
            },

            Identifier(node, state, c) {
                switch (node.name) {
                    case 'undefined':
                        emit('Value::Undefined');
                        break;
                    case 'null':
                        emit("Value::Json(json!(null))");
                        break;

                    case 'true':
                    case 'false':
                        emit(node.name);
                        switch (state.asType) {
                            case 'bool':
                            case undefined:
                                break;

                            case 'value':
                                emit(".into()");
                                break;

                            default:
                                throw new Error(`Unsupported type ${state.asType}`);
                        }
                        break;

                    default:
                        if (isAllCaps(node.name)) {
                            emit(node.name);
                            switch (state.asType) {
                                case undefined:
                                case 'value':
                                    emit(".into()");
                                    break;

                                default:
                                    throw new Error(`Unsupported type ${state.asType}`);
                            }
                        } else {
                            switch (state.asType) {
                                case 'property':
                                    emit('"');
                                    break;

                                case undefined:
                                case 'bool':
                                case 'rvalue':
                                case 'value':
                                case 'member':
                                    break;

                                default:
                                    throw new Error(`Unsupported type ${state.asType}`);
                            }
                            let isValueType = false;
                            switch (node.name) {
                                case 'i':
                                case 'j':
                                    if (state.asType === 'value') {
                                        emit("Value::from(");
                                    }
                                    emit(node.name);
                                    isValueType = true;
                                    break;

                                case 'length':
                                    if (state.asType === 'value') {
                                        emit("Value::from(");
                                    }
                                    emit('len()');
                                    isValueType = true;
                                    break;

                                default:
                                    emit(transformIdentifier(node.name));
                                    break;
                            }
                            switch (state.asType) {
                                case 'property':
                                    emit('".into()');
                                    break;

                                case 'bool':
                                    emit(".is_truthy()");
                                    break;

                                case 'rvalue':
                                case 'value':
                                    if (isValueType) {
                                        emit(")");
                                    } else {
                                        emit(".clone()");
                                    }
                                    break;

                                case 'member':
                                case undefined:
                                    break;

                                default:
                                    throw new Error(`Unsupported type ${state.asType}`);
                            }
                        }
                }
            },

            ReturnStatement(node, state, c) {
                emit("return ");
                c(node.argument, asType(state, 'value'));
            },

            UpdateExpression(node, state, c) {
                c(node.argument, asType(state));
                switch (node.operator) {
                    case "++":
                        emit(" += 1");
                        break;
                    default:
                        throw new Error("Unsupported update operator: " + node.operator);
                }
            },

            ArrayExpression(node, state, c) {
                if (node.elements.length === 0) {
                    emit("Value::new_array()");
                    return;
                }

                emit("Value::Json(serde_json::Value::Array(vec![");
                for (let i = 0; i < node.elements.length; i++) {
                    const element = node.elements[i];
                    if (!element) {
                        emit("Value::Undefined");
                    } else {
                        c(element, asType(state, 'value'));
                        emit(".into()");
                    }
                    if (i < node.elements.length - 1) {
                        emit(", ");
                    }
                }
                emit("]))");
            },

            ForStatement(node, state, c) {
                c(node.init, asType({
                    ...state,
                    parent: node
                }));
                emit(";\n");
                indent(state);
                emit("while ");
                c(node.test, asType(state));
                emit(" ");

                const updateOutput = withNewOutput(node.update, state, c);
                c(node.body, asType({
                    ...state,
                    indentLevel: state.indentLevel + 1,
                    appendBlock: updateOutput + ";"
                }));
            },

            ContinueStatement(node, state, c) {
                emit("continue");
            },

            ObjectExpression(node, state, c) {
                if (node.properties.length === 0) {
                    emit("Value::new_object()");
                    return;
                }

                emit("Value::Json(json!({");
                emit("\n");
                const state1 = asType({
                    ...state,
                    indentLevel: state.indentLevel + 1
                });
                for (let i = 0; i < node.properties.length; i++) {
                    const node1 = node.properties[i];
                    switch (node1.type) {
                        case 'Property':
                            indent(state1);
                            emit(`"${node1.key.value}": `);
                            c(node1.value, asType(state));
                            if (i < node.properties.length - 1) {
                                emit(`,\n`);
                            }
                            break;
                        default:
                            throw new Error('Unsupported object expression type: ' + node1.type);
                    }
                }
                emit("\n");
                indent(state);
                emit("}))");
            },

            Program(node, state, c) {
                for (const child of node.body) {
                    c(child, state);
                }
            },
        }, {});

        return currentOutput.value + "\n";
    }
}