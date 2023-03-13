import { ChatCardSkeleton } from './ChatCardSkeleton';

export const ChatLoading = () => {
	const filler = Array.from({ length: 3 }, (_, i) => i);

	return (
		<>
			{filler.map((_, i) => (
				<ChatCardSkeleton key={i}>
					<div role="status" className="max-w-sm animate-pulse">
						<div className="h-5 max-w-[80px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
						<div className="mt-6 h-3 max-w-[360px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
						<div className="mt-2 h-3 max-w-[360px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
						<div className="mt-4 flex space-x-2">
							<div className="h-3 w-8 shrink rounded-full bg-gray-300 dark:bg-gray-700"></div>
							<div className="h-3 w-16 rounded-full bg-gray-300 dark:bg-gray-700"></div>
						</div>
						<span className="sr-only">Loading...</span>
					</div>
				</ChatCardSkeleton>
			))}
		</>
	);
};
