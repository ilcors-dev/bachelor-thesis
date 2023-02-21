interface Props {
	message: Message;
	className?: string;
}

export const Message = ({ message, className }: Props) => {
	return (
		<li
			className={`block max-w-sm cursor-pointer rounded-lg border border-gray-200 p-6 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700 ${className}`}
		>
			<p>{message.text}</p>
			<p className="text-xs text-gray-500 dark:text-gray-400">
				{new Date(message.created_at).toLocaleString()}
			</p>
		</li>
	);
};
