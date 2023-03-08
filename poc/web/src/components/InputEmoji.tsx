import EmojiPicker, { EmojiClickData } from 'emoji-picker-react';

interface Props {
	className?: string;
	handleEmojiClick: (emoji: EmojiClickData, event: MouseEvent) => void;
}

export const InputEmoji = ({ className, handleEmojiClick }: Props) => {
	return (
		<div className={`${className}`}>
			<button
				data-popover-target="popover-default"
				type="button"
				className="h-full rounded-lg bg-blue-700 px-5 py-2.5 text-center text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
			>
				<span role="img" aria-label="emoji">
					😃
				</span>
			</button>
			<div
				data-popover
				id="popover-default"
				role="tooltip"
				className="invisible absolute z-10 inline-block w-64 rounded-lg border border-gray-200 bg-white text-sm font-light text-gray-500 opacity-0 shadow-sm transition-opacity duration-300 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-400"
			>
				<EmojiPicker onEmojiClick={handleEmojiClick} />
			</div>
		</div>
	);
};
