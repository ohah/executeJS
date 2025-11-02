import { Tab, usePlaygroundStore } from '@/features/playground';
import { Cross1Icon } from '@radix-ui/react-icons';
import { Controller, SubmitHandler, useFormContext } from 'react-hook-form';

interface TabTitleModalProps {
  open: boolean;
  onClose: () => void;
}

export const TabTitleModal: React.FC<TabTitleModalProps> = ({
  open,
  onClose,
}) => {
  const { setTabTitle } = usePlaygroundStore();

  const { control, handleSubmit } = useFormContext<Pick<Tab, 'id' | 'title'>>();

  const handleChangeTabTitle: SubmitHandler<Pick<Tab, 'id' | 'title'>> = ({
    id,
    title,
  }) => {
    setTabTitle({ tabId: id, title: title.trim() });
    onClose();
  };

  if (!open) return null;

  return (
    <div className="fixed top-0 right-0 bottom-0 left-0 z-10 flex items-center justify-center">
      <div
        className="absolute top-0 right-0 bottom-0 left-0 bg-white opacity-20"
        onClick={onClose}
      />

      <div className="absolute flex flex-col p-6 border border-black rounded-2xl bg-gray-950 shadow-[2px_4px_4px_rgba(0,0,0,0.3)]">
        <div className="flex justify-between items-center">
          <strong className="text-lg">Change tab title</strong>
          <button type="button" onClick={onClose} className="cursor-pointer">
            <Cross1Icon />
          </button>
        </div>

        <Controller
          control={control}
          name="title"
          render={({ field: { value, onChange } }) => {
            return (
              <input
                type="text"
                autoFocus
                value={value}
                onChange={onChange}
                className="py-1 px-2 mt-4 mb-3 rounded-md bg-gray-900"
              />
            );
          }}
        />

        <button
          type="submit"
          onClick={handleSubmit(handleChangeTabTitle)}
          className="p-1 rounded-md bg-gray-500 cursor-pointer"
        >
          Save
        </button>
      </div>
    </div>
  );
};
