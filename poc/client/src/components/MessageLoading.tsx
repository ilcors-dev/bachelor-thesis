export const MessageLoading = () => {
	return (
		<div className="block max-w-sm cursor-pointer rounded-lg border border-gray-200 bg-white p-6 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
			<div role="status" className="max-w-sm animate-pulse">
				<div className="mb-2.5 h-3 max-w-[360px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
				<div className="mb-2.5 h-3 rounded-full bg-gray-300 dark:bg-gray-700"></div>
				<div className="mb-2.5 h-3 max-w-[330px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
				<div className="mb-2.5 h-3 max-w-[300px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
				<div className="h-3 max-w-[360px] rounded-full bg-gray-300 dark:bg-gray-700"></div>
				<span className="sr-only">Loading...</span>
			</div>
		</div>
	);
};
