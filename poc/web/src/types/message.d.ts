interface Message {
	id: number;
	ulid: string;
	text: string;
	created_at: string;
	updated_at: string;
}

interface MessageWithSender extends Message {
	sender: {
		name: string;
		emoji: string;
	};
}
