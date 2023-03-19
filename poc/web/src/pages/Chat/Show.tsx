import autoAnimate from '@formkit/auto-animate';
import axios from 'axios';
import { Tooltip } from 'flowbite-react';
import { useEffect, useRef, useState } from 'react';
import { useQuery } from 'react-query';
import { Navigate, useParams } from 'react-router-dom';
import { Message } from '../../components/Message';
import { MessageInput } from '../../components/MessageInput';
import { MessageLoading } from '../../components/MessageLoading';
import { OnlineUsers } from '../../components/OnlineUsers';
import { useSession } from '../../hooks/useSession';

export const Show = () => {
	const { chatId } = useParams<{ chatId: string }>();

	if (!chatId) {
		return <Navigate to="/error" />;
	}

	const currentSession = useSession().offlineGet();
	const [firstLoaded, setFirstLoaded] = useState(false);
	const [lastMessageLoadedId, setLastMessageLoadedId] = useState<number | null>(
		null
	);
	const messagesContainer = useRef(null);
	const bottomRef = useRef(null);

	useEffect(() => {
		messagesContainer.current && autoAnimate(messagesContainer.current);
	}, [messagesContainer]);

	const { isLoading, data, error, refetch } = useQuery<MessageWithSender[]>(
		['messages'],
		async () => {
			let params: { [key: string]: number | string } = {
				chat_id: chatId,
			};

			if (!firstLoaded && lastMessageLoadedId) {
				params = {
					fetch_from_message_id: lastMessageLoadedId,
					...params,
				};
			}

			const response = await axios.get('/api/messages', {
				params: {
					chat_id: chatId,
				},
			});

			setFirstLoaded(true);

			return response.data as MessageWithSender[];
		},
		{
			refetchInterval: 1000,
			// onSuccess: (messages) => {
			// 	if (!messages || messages.length === 0) {
			// 		return;
			// 	}

			// 	setTimeout(() => {
			// 		(bottomRef.current as unknown as HTMLDivElement)?.scrollIntoView({
			// 			behavior: 'smooth',
			// 		});
			// 	}, 50);
			// },
		}
	);

	useEffect(() => {
		if (!data) {
			return;
		}

		setLastMessageLoadedId(data[data.length - 1]?.id ?? null);
	}, [data]);

	return (
		<>
			<div className="flex h-full flex-col py-8">
				<h1 className="text-4xl font-bold">Chat</h1>
				{isLoading && <MessageLoading />}
				<div className="my-2.5 h-full shrink grow basis-0 overflow-y-auto">
					<ul>
						{data &&
							data.map((message, i) => (
								<div
									className={`flex items-center space-x-2 ${
										message.sender.name === currentSession?.name
											? 'justify-end'
											: ''
									}`}
								>
									{message.sender.name !== currentSession?.name && (
										<Tooltip placement="right" content={message.sender.name}>
											<p className="cursor-pointer text-xl">
												{message.sender.emoji}
											</p>
										</Tooltip>
									)}
									<Message key={i} className={`my-1`} message={message} />
								</div>
							))}
						<div ref={bottomRef}></div>
					</ul>
				</div>
				<div className="shrink-0 grow-0 basis-auto">
					<MessageInput
						className="w-full"
						onSubmit={() => {
							setTimeout(() => {
								(
									bottomRef.current as unknown as HTMLDivElement
								)?.scrollIntoView({
									behavior: 'smooth',
								});
							}, 50);
						}}
					/>
				</div>
			</div>
			<OnlineUsers />
		</>
	);
};
