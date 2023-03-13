interface Props {
	children?: React.ReactNode;
	className?: string;
}

export const ChatCardSkeleton = ({ children, className }: Props) => {
	return (
		<div
			className={`block w-full cursor-pointer rounded-lg border border-gray-200 bg-white p-6 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700 ${className}`}
		>
			{children}
		</div>
	);
};
