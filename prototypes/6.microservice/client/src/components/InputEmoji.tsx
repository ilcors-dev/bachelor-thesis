import EmojiPicker from 'emoji-picker-react';
import { useState } from 'react';
import { usePopper } from 'react-popper';

interface Props {
	className?: string;
}

export const InputEmoji = ({ className }: Props) => {
	const [referenceElement, setReferenceElement] = useState(null);
	const [popperElement, setPopperElement] = useState(null);
	const [arrowElement, setArrowElement] = useState(null);
	const { styles, attributes } = usePopper(referenceElement, popperElement, {
		modifiers: [{ name: 'arrow', options: { element: arrowElement } }],
	});

	return (
		<div className={`${className}`}>
			<button
				type="button"
				ref={setReferenceElement}
				className="h-full w-max shrink rounded-lg bg-blue-700 px-4 py-2.5 text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
			>
				<span role="img" aria-label="emoji">
					😃
				</span>
			</button>
			<div ref={setPopperElement} style={styles.popper} {...attributes.popper}>
				<p>e</p>
				<div ref={setArrowElement} style={styles.arrow} />
			</div>
		</div>
	);
};
