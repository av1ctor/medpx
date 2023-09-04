import React from 'react';
import { attachProps, createForwardRef, dashToPascalCase, isCoveredByReact, mergeRefs, } from '../utils';

export const createReactComponent = (tagName, ReactComponentContext, manipulatePropsFunction) => {
    const displayName = dashToPascalCase(tagName);
    const ReactComponent = class extends React.Component {
        componentEl;
        setComponentElRef = (element) => {
            this.componentEl = element;
        };
        constructor(props) {
            super(props);
        }
        componentDidMount() {
            this.componentDidUpdate(this.props);
        }
        componentDidUpdate(prevProps) {
            attachProps(this.componentEl, this.props, prevProps);
        }
        render() {
            const { children, forwardedRef, style, className, ref, ...cProps } = this.props;
            let propsToPass = Object.keys(cProps).reduce((acc, name) => {
                if (name.indexOf('on') === 0 && name[2] === name[2].toUpperCase()) {
                    const eventName = name.substring(2).toLowerCase();
                    if (typeof document !== 'undefined' && isCoveredByReact(eventName, document)) {
                        acc[name] = cProps[name];
                    }
                }
                else {
                    acc[name] = cProps[name];
                }
                return acc;
            }, {});
            if (manipulatePropsFunction) {
                propsToPass = manipulatePropsFunction(this.props, propsToPass);
            }
            let newProps = {
                ...propsToPass,
                ref: mergeRefs(forwardedRef, this.setComponentElRef),
                style,
            };
            return React.createElement(tagName, newProps, children);
        }
        static get displayName() {
            return displayName;
        }
    };
    // If context was passed to createReactComponent then conditionally add it to the Component Class
    if (ReactComponentContext) {
        ReactComponent.contextType = ReactComponentContext;
    }
    return createForwardRef(ReactComponent, displayName);
};