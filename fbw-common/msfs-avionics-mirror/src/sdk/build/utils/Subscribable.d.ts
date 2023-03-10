/**
 * An item which allows others to subscribe to be notified of changes in its state.
 */
export interface Subscribable<T> {
    /**
     * Gets the state of this subscribable.
     * @returns the state of this subscribable.
     */
    get(): T;
    /**
     * Subscribes to changes in this subscribable's state.
     * @param fn A function which is called when this subscribable's state changes.
     * @param initialNotify Whether to immediately invoke the callback function when it is subscribed with this
     * observable's current state.
     */
    sub(fn: (value: T) => void, initialNotify?: boolean): void;
    /**
     * Unsubscribes a callback function from this subscribable.
     * @param fn The function to unsubscribe.
     */
    unsub(fn: (value: T) => void): void;
    /**
     * Maps this subscribable to a new subscribable.
     * @param fn The function to use to map to the new subscribable.
     * @param equalityFunc The function to use to check for equality between mapped values. Defaults to the strict
     * equality comparison (`===`).
     * @returns The mapped subscribable.
     */
    map<M>(fn: (input: T, previousVal?: M) => M, equalityFunc?: ((a: M, b: M) => boolean)): MappedSubscribable<M>;
    /**
     * Maps this subscribable to a new subscribable with a persistent, cached value which is mutated when it changes.
     * @param fn The function to use to map to the new subscribable.
     * @param equalityFunc The function to use to check for equality between mapped values.
     * @param mutateFunc The function to use to change the value of the mapped subscribable.
     * @param initialVal The initial value of the mapped subscribable.
     * @returns The mapped subscribable.
     */
    map<M>(fn: (input: T, previousVal?: M) => M, equalityFunc: ((a: M, b: M) => boolean), mutateFunc: ((oldVal: M, newVal: M) => void), initialVal: M): MappedSubscribable<M>;
}
/**
 * A subscribable which is mapped from another subscribable.
 */
export interface MappedSubscribable<T> extends Subscribable<T> {
    /**
     * Destroys the subscription to the parent subscribable.
     */
    destroy(): void;
}
/**
 * Types of subscribable array change event.
 */
export declare enum SubscribableArrayEventType {
    /** An element was added. */
    Added = "Added",
    /** An element was removed. */
    Removed = "Removed",
    /** The array was cleared. */
    Cleared = "Cleared"
}
/**
 * An array which allows others to subscribe to be notified of changes in its state.
 */
export interface SubscribableArray<T> {
    /** The length of this array. */
    readonly length: number;
    /**
     * Retrieves an element from this array.
     * @param index The index of the element.
     * @returns the element at the specified index.
     * @throws Error if index is out of bounds.
     */
    get(index: number): T;
    /**
     * Attempts to retrieve an element from this array.
     * @param index The index of the element.
     * @returns the element at the specified index, or undefined if index is out of bounds.
     */
    tryGet(index: number): T | undefined;
    /**
     * Gets a read-only version of this array.
     * @returns a read-only version of this array.
     */
    getArray(): readonly T[];
    /**
     * Subscribes to this array with a callback function. The function will be called whenever this array changes.
     * @param fn A function which is called when this array's state changes.
     * @param initialNotify Whether to immediately notify the callback function after it is subscribed. False by default.
     */
    sub(fn: (index: number, type: SubscribableArrayEventType, item: T | readonly T[] | undefined, array: readonly T[]) => void, initialNotify?: boolean): void;
    /**
     * Unsubscribes a callback function from this array.
     * @param fn The function to unsubscribe.
     */
    unsub(fn: (index: number, type: SubscribableArrayEventType, item: T | readonly T[] | undefined, array: readonly T[]) => void): void;
}
/**
 * An abstract implementation of a subscribable which allows adding, removing, and notifying subscribers.
 */
export declare abstract class AbstractSubscribable<T> implements Subscribable<T> {
    /**
     * Checks if two values are equal using the strict equality operator.
     * @param a The first value.
     * @param b The second value.
     * @returns whether a and b are equal.
     */
    static readonly DEFAULT_EQUALITY_FUNC: (a: any, b: any) => boolean;
    protected readonly subs: {
        (v: T): void;
    }[];
    /** @inheritdoc */
    abstract get(): T;
    /** @inheritdoc */
    sub(fn: (v: T) => void, initialNotify?: boolean): void;
    /** @inheritdoc */
    unsub(fn: (v: T) => void): void;
    /**
     * Notifies subscribers that this subscribable's value has changed.
     */
    protected notify(): void;
    /**
     * Maps this subscribable to a new subscribable.
     * @param fn The function to use to map to the new subscribable.
     * @param equalityFunc The function to use to check for equality between mapped values. Defaults to the strict
     * equality comparison (`===`).
     * @returns The mapped subscribable.
     */
    abstract map<M>(fn: (input: T, previousVal?: M) => M, equalityFunc?: (a: M, b: M) => boolean): MappedSubscribable<M>;
    /**
     * Maps this subscribable to a new subscribable with a persistent, cached value which is mutated when it changes.
     * @param fn The function to use to map to the new subscribable.
     * @param equalityFunc The function to use to check for equality between mapped values. Defaults to the strict
     * equality comparison (`===`).
     * @param mutateFunc The function to use to change the value of the mapped subscribable.
     * @param initialVal The initial value of the mapped subscribable.
     * @returns The mapped subscribable.
     */
    abstract map<M>(fn: (input: T, previousVal?: M) => M, equalityFunc: (a: M, b: M) => boolean, mutateFunc: (oldVal: M, newVal: M) => void, initialVal: M): MappedSubscribable<M>;
}
/**
 * Utility class for generating common mapping functions.
 */
export declare class SubscribableMapFunctions {
    /**
     * Generates a function which maps an input boolean to its negation.
     * @returns A function which maps an input boolean to its negation.
     */
    static not<T extends boolean>(): (input: T, currentVal?: T) => boolean;
    /**
     * Generates a function which maps an input number to its negation.
     * @returns A function which maps an input number to its negation.
     */
    static negate<T extends number>(): (input: T, currentVal?: T) => number;
    /**
     * Generates a function which maps an input number to its absolute value.
     * @returns A function which maps an input number to its absolute value.
     */
    static abs<T extends number>(): (input: T, currentVal?: T) => number;
    /**
     * Generates a function which maps an input number to a rounded version of itself at a certain precision.
     * @param precision The precision to which to round the input.
     * @returns A function which maps an input number to a rounded version of itself at the specified precision.
     */
    static withPrecision<T extends number>(precision: number): (input: T, currentVal?: T) => number;
    /**
     * Generates a function which maps an input number to itself if and only if it differs from the previous mapped value
     * by a certain amount, and to the previous mapped value otherwise.
     * @param threshold The minimum difference between the input and the previous mapped value required to map the input
     * to itself.
     * @returns A function which maps an input number to itself if and only if it differs from the previous mapped value
     * by the specified amount, and to the previous mapped value otherwise.
     */
    static changedBy<T extends number>(threshold: number): (input: T, currentVal?: T) => number;
    /**
     * Generates a function which maps an input number to itself up to a maximum frequency, and to the previous mapped
     * value otherwise.
     * @param freq The maximum frequency at which to map the input to itself, in Hertz.
     * @param timeFunc A function which gets the current time in milliseconds. Defaults to `Date.now()`.
     * @returns A function which maps an input number to itself up to the specified maximum frequency, and to the
     * previous mapped value otherwise.
     */
    atFrequency<T>(freq: number, timeFunc?: () => number): (input: T, currentVal?: T) => T;
}
//# sourceMappingURL=Subscribable.d.ts.map