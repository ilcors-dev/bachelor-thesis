import { UserIcon } from '@heroicons/react/20/solid';
import { Tooltip } from 'flowbite-react';
import { useSession } from '../hooks/useSession';
import { greater, is } from '../hooks/useTailwindBreakpoints';

export const Profile = () => {
	const me = useSession().offlineGet();

	return (
		<div className="group my-auto flex h-min cursor-pointer items-center justify-center rounded-lg border border-transparent bg-white p-3 text-center text-sm font-medium text-black hover:bg-gray-100 focus:z-10 focus:ring-4 focus:!ring-2 focus:ring-blue-300 disabled:hover:bg-blue-700 dark:bg-blue-600 dark:text-white dark:hover:bg-blue-700 dark:focus:ring-blue-800 dark:disabled:hover:bg-blue-600 sm:p-2 sm:text-base">
			{!greater('md') && (
				<Tooltip placement="bottom" content={`${me?.emoji} ${me?.name}`}>
					<UserIcon className="h-4 w-4" />
				</Tooltip>
			)}
			{greater('md') && (
				<span>
					{me?.emoji} {me?.name}
				</span>
			)}
		</div>
	);
};
