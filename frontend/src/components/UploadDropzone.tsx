import { useCallback, useState } from 'react';
import { UploadCloud, Loader2 } from 'lucide-react';
import { endpoints } from '../lib/api';
import clsx from 'clsx';

interface UploadDropzoneProps {
  onSuccess: (data: any) => void;
}

export function UploadDropzone({ onSuccess }: UploadDropzoneProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setIsDragging(true);
    } else if (e.type === 'dragleave') {
      setIsDragging(false);
    }
  }, []);

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
    setError(null);

    const files = e.dataTransfer.files;
    if (files && files[0]) {
      await uploadFile(files[0]);
    }
  }, []);

  const handleChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setError(null);
    if (e.target.files && e.target.files[0]) {
      await uploadFile(e.target.files[0]);
    }
  };

  const uploadFile = async (file: File) => {
    if (!file.name.endsWith('.csv')) {
      setError('Please upload a CSV file.');
      return;
    }

    setIsUploading(true);
    try {
      const response = await endpoints.analyze(file);
      onSuccess(response.data);
    } catch (err) {
      console.error(err);
      setError('Failed to analyze file. Please try again.');
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div
      className={clsx(
        'relative border-2 border-dashed rounded-lg p-12 text-center transition-colors',
        isDragging
          ? 'border-blue-500 bg-blue-50'
          : 'border-gray-300 hover:border-gray-400 bg-gray-50'
      )}
      onDragEnter={handleDrag}
      onDragLeave={handleDrag}
      onDragOver={handleDrag}
      onDrop={handleDrop}
    >
      <input
        type="file"
        className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
        onChange={handleChange}
        accept=".csv"
        disabled={isUploading}
      />
      
      <div className="flex flex-col items-center justify-center space-y-4">
        {isUploading ? (
          <>
            <Loader2 className="w-12 h-12 text-blue-500 animate-spin" />
            <p className="text-lg font-medium text-gray-700">Analyzing trips...</p>
          </>
        ) : (
          <>
            <div className="p-4 bg-white rounded-full shadow-sm">
              <UploadCloud className="w-8 h-8 text-blue-500" />
            </div>
            <div>
              <p className="text-lg font-medium text-gray-900">
                Click to upload or drag and drop
              </p>
              <p className="text-sm text-gray-500">CSV files only (max 10MB)</p>
            </div>
          </>
        )}
      </div>

      {error && (
        <div className="absolute bottom-4 left-0 right-0">
          <p className="text-sm text-red-600 font-medium">{error}</p>
        </div>
      )}
    </div>
  );
}
