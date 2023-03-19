import { useSession } from '../hooks/useSession';

export const Profile = () => {
	const me = useSession().offlineGet();

	return (
		<div className="group my-auto flex h-min cursor-pointer items-center justify-center rounded-lg border border-transparent bg-white p-2 text-center font-medium text-black text-white hover:bg-gray-100 focus:z-10 focus:ring-4 focus:!ring-2 focus:ring-blue-300 disabled:hover:bg-blue-700 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 dark:disabled:hover:bg-blue-600">
			<p>
				{me?.emoji} {me?.name}
			</p>
		</div>
	);
};
